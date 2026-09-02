use std::borrow::Cow;
use std::collections::BTreeSet;

use chrono::{DateTime, FixedOffset};

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use crate::plmn::{PACKED_BCD_LEN, decode_packed_bcd};
use log::{debug, error};

use pycrate_rs::nas::generated::emm::emm_attach_reject::EMMCauseEMMCause as AttachRejectEMMCause;
use pycrate_rs::nas::generated::emm::emm_attach_request::TAI;
use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};
use telcom_parser::lte_rrc::{MCC_MNC_Digit, PLMN_Identity, PLMN_IdentityList};
use telcom_parser::lte_rrc::{
    /* DL_DCCH_MessageType, DL_DCCH_MessageType_c1,*/ UL_CCCH_MessageType,
    UL_CCCH_MessageType_c1,
};

const TIMEOUT_THRESHHOLD: usize = 50;

#[derive(PartialEq, Debug)]
pub enum State {
    Unattached,
    AttachRequest,
    IdentityRequest,
    AuthAccept,
    Disconnect,
    LikelyValidAttachReject,
}

pub struct ImsiRequestedAnalyzer {
    state: State,
    timeout_counter: usize,
    flag: Option<Event>,
    likely_enb_plmns: Vec<String>,
    likely_ue_plmn: Option<String>,
    /// From the SIM (EF_HPLMNwAcT). Empty means unknown, not a mismatch.
    home_plmn: BTreeSet<String>,
}

impl Default for ImsiRequestedAnalyzer {
    fn default() -> Self {
        Self::new(BTreeSet::new())
    }
}

impl ImsiRequestedAnalyzer {
    pub fn new(home_plmn: BTreeSet<String>) -> Self {
        Self {
            state: State::Unattached,
            timeout_counter: 0,
            flag: None,
            likely_enb_plmns: vec![],
            likely_ue_plmn: None,
            home_plmn,
        }
    }

    /// Whether the tower broadcasts a PLMN this subscriber has a legitimate
    /// relationship with.
    fn enb_is_home_network(&self) -> bool {
        // home_plmn and likely_ue_plmn can disagree, so we check both. Incorrectly returning true
        // is safer than Incorrectly returning false, as severity is higher on the home network.
        self.likely_ue_plmn
            .as_ref()
            .is_some_and(|p| self.likely_enb_plmns.contains(p))
            || self
                .home_plmn
                .iter()
                .any(|p| self.likely_enb_plmns.contains(p))
    }

    fn transition(&mut self, next_state: State, packet_num: usize) {
        match (&self.state, &next_state) {
            // Reset timeout on successful auth
            (_, State::AuthAccept) => {
                debug!(
                    "reset timeout counter at {} due to auth accept (frame {})",
                    self.timeout_counter, packet_num
                );
                self.timeout_counter = 0;
            }

            // IMSI or IMEI requested after auth accept
            (State::AuthAccept, State::IdentityRequest) => {
                self.flag = Some(Event {
                    event_type: EventType::High,
                    message: "Identity requested after auth request".to_string(),
                });
            }

            // Unexpected IMSI without AttachRequest
            (State::Disconnect, State::IdentityRequest) => {
                self.flag = Some(Event {
                    event_type: EventType::High,
                    message: "Identity requested without Attach Request".to_string(),
                });
            }

            // Expected AttachReject for inactive SIMs
            (State::IdentityRequest, State::LikelyValidAttachReject) => {
                self.flag = Some(Event {
                    event_type: EventType::Low,
                    message: "Identity requested without authentication but its likely a false positive unless your SIM card has an active plan".to_string(),
                });
            }

            // IMSI to Disconnect without AuthAccept
            (State::IdentityRequest, State::Disconnect) => {
                if self.enb_is_home_network() {
                    self.flag = Some(Event {
                        event_type: EventType::High,
                        message: "Disconnected after Identity Request without Auth Accept on home network!".to_string(),
                    });
                } else {
                    let enb_plmn_string = if self.likely_enb_plmns.is_empty() {
                        "Unknown"
                    } else {
                        &self.likely_enb_plmns.join(", ")
                    };
                    let ue_plmn_string = self.likely_ue_plmn.as_deref().unwrap_or("Unknown");
                    let home_plmn_string = if self.home_plmn.is_empty() {
                        "Unknown".to_string()
                    } else {
                        self.home_plmn
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    };

                    self.flag = Some(Event {
                        event_type: EventType::Low,
                        message: format!(
                            "Disconnected after Identity Request without Auth Accept, but this could be a false positive roaming issue - Tower PLMN: {}, UE PLMN: {}, SIM home PLMN: {}",
                            enb_plmn_string, ue_plmn_string, home_plmn_string,
                        ),
                    });
                }
            }

            (_, State::IdentityRequest) => {
                self.timeout_counter = 0;
            }

            // All other transitions proceeed
            _ => {
                debug!(
                    "Transition from {:?} to {:?} at {}",
                    self.state, next_state, packet_num
                );
            }
        }

        // LikelyValidAttachReject is a special case of Disconnect so after handling any special
        // behavior above, we transition to the standard Disconnect state.
        if next_state == State::LikelyValidAttachReject {
            self.state = State::Disconnect;
        } else {
            self.state = next_state;
        }
    }

    // Sometimes an ENB can have multiple PLMNS
    fn format_plmn_list(&self, plmn_list: &PLMN_IdentityList) -> Vec<String> {
        plmn_list
            .0
            .iter()
            .map(|info| self.plmn_identity_to_str(&info.plmn_identity))
            .collect()
    }

    // PLMN is represented in two very different ways in the LTE spec so we need
    // two very different functions to decode them. I hate this.
    fn plmn_identity_to_str(&self, plmn: &PLMN_Identity) -> String {
        let mcc_digits: String = plmn
            .mcc
            .as_ref()
            .map(|mcc| {
                mcc.0
                    .iter()
                    .map(|MCC_MNC_Digit(n)| n.to_string())
                    .collect::<String>()
            })
            .unwrap_or_default();

        let mnc_digits: String = plmn
            .mnc
            .0
            .iter()
            .map(|MCC_MNC_Digit(n)| n.to_string())
            .collect::<String>();

        format!("{}-{}", mcc_digits, mnc_digits)
    }

    fn tai_to_plmn_str(&self, maybe_tai: Option<&TAI>) -> Option<String> {
        let plmn = &maybe_tai?.plmn;
        if plmn.len() != PACKED_BCD_LEN {
            error!("TAI.plmn vector has unexpected length of {}", plmn.len());
            return None;
        }
        decode_packed_bcd(plmn)
    }
}

impl Analyzer for ImsiRequestedAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Identity (IMSI or IMEI) requested in suspicious manner")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "Tests whether the ME sends an Identity Request NAS message without either an associated attach request or auth accept message",
        )
    }

    fn get_version(&self) -> u32 {
        5
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
        _timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        // Set the enodeb plmn to the last sib1 we got, we should improve this once we have PCI data, this
        // is a naive approach.
        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(sch_msg) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(c1) = &sch_msg.message
            && let BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) = c1
        {
            let plmn = &sib1.cell_access_related_info.plmn_identity_list;
            self.likely_enb_plmns = self.format_plmn_list(plmn);

            return None;
        }

        if let InformationElement::LTE(inner) = ie {
            match &**inner {
                LteInformationElement::NAS(payload) => match payload {
                    NASMessage::EMMMessage(EMMMessage::EMMAttachRequest(request)) => {
                        let maybe_plmn = self.tai_to_plmn_str(request.old_tai.inner.as_ref());
                        if maybe_plmn.is_some() {
                            self.likely_ue_plmn = maybe_plmn;
                        }
                        self.transition(State::AttachRequest, packet_num);
                    }
                    NASMessage::EMMMessage(EMMMessage::EMMExtServiceRequest(_)) => {
                        self.transition(State::AttachRequest, packet_num);
                    }
                    NASMessage::EMMMessage(EMMMessage::EMMIdentityRequest(_)) => {
                        self.transition(State::IdentityRequest, packet_num);
                    }
                    NASMessage::EMMMessage(EMMMessage::EMMAttachComplete(_))
                    | NASMessage::EMMMessage(EMMMessage::EMMAuthenticationResponse(_)) => {
                        self.transition(State::AuthAccept, packet_num);
                    }
                    NASMessage::EMMMessage(EMMMessage::EMMServiceReject(_))
                    | NASMessage::EMMMessage(EMMMessage::EMMDetachRequestMO(_))
                    | NASMessage::EMMMessage(EMMMessage::EMMDetachRequestMT(_))
                    | NASMessage::EMMMessage(EMMMessage::EMMTrackingAreaUpdateReject(_)) => {
                        self.transition(State::Disconnect, packet_num);
                    }
                    NASMessage::EMMMessage(EMMMessage::EMMAttachReject(reject)) => {
                        if reject.emm_cause.inner
                            == AttachRejectEMMCause::EPSServicesAndNonEPSServicesNotAllowed
                        {
                            self.transition(State::LikelyValidAttachReject, packet_num);
                        } else {
                            self.transition(State::Disconnect, packet_num);
                        }
                    }
                    _ => {}
                },

                LteInformationElement::UlCcch(rrc_payload) => match rrc_payload.message {
                    UL_CCCH_MessageType::C1(UL_CCCH_MessageType_c1::RrcConnectionRequest(_))
                    | UL_CCCH_MessageType::C1(
                        UL_CCCH_MessageType_c1::RrcConnectionReestablishmentRequest(_),
                    ) => {
                        self.transition(State::AttachRequest, packet_num);
                    }
                    _ => {}
                },

                // This causes two messages in the event of a false positive when we should always get an attach reject anyway so
                // I'm commentingit out until I figure out a smarter way to deal with it.
                /*
                LteInformationElement::DlDcch(rrc_payload) => {
                    if let DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionRelease(
                        _,
                    )) = rrc_payload.message
                    {
                        self.transition(State::Disconnect, packet_num)
                    }
                }
                */
                _ => {}
            }
        };

        if self.state == State::IdentityRequest {
            self.timeout_counter += 1;
            debug!(
                "timeout: counter {}, packet: {}",
                self.timeout_counter, packet_num
            );
            if self.timeout_counter >= TIMEOUT_THRESHHOLD {
                self.flag = Some(Event {
                    event_type: EventType::Informational {},
                    message: "Identity request happened without auth request followup".to_string(),
                });
                self.timeout_counter = 0;
            }
        }

        self.flag.take()
    }
}
