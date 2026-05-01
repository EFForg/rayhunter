use std::borrow::Cow;

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use log::debug;

use pycrate_rs::nas::generated::emm::emm_attach_reject::EMMCauseEMMCause as AttachRejectEMMCause;
use pycrate_rs::nas::generated::emm::emm_attach_request::TAI;
use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};
use telcom_parser::lte_rrc::{
    /* DL_DCCH_MessageType, DL_DCCH_MessageType_c1,*/ UL_CCCH_MessageType, UL_CCCH_MessageType_c1,
};
use telcom_parser::lte_rrc::{MCC_MNC_Digit, PLMN_Identity, PLMN_IdentityList};

const TIMEOUT_THRESHHOLD: usize = 50;

#[derive(PartialEq, Debug)]
pub enum State {
    Unattached,
    AttachRequest,
    IdentityRequest,
    AuthAccept,
    Disconnect,
}

pub struct ImsiRequestedAnalyzer {
    state: State,
    timeout_counter: usize,
    flag: Option<Event>,
    likely_enb_plmn: String,
    likely_ue_plmn: String,
}

impl Default for ImsiRequestedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsiRequestedAnalyzer {
    pub fn new() -> Self {
        Self {
            state: State::Unattached,
            timeout_counter: 0,
            flag: None,
            // You will likely wonder why this isn't an Option<PLMN{mcc: u32, mnc: u32}>
            // The answer is that I like strings.
            likely_enb_plmn: "Unknown".to_string(),
            likely_ue_plmn: "Unknown".to_string(),
        }
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

            // IMSI to Disconnect without AuthAccept
            (State::IdentityRequest, State::Disconnect) => {
                if self.likely_enb_plmn == self.likely_ue_plmn {
                    self.flag = Some(Event {
                        event_type: EventType::High,
                        message: "Disconnected after Identity Request without Auth Accept on home network!".to_string(),
                    });
                } else {
                    self.flag = Some(Event {
                        event_type: EventType::Low,
                        message: format!(
                            "Disconnected after Identity Request without Auth Accept, but this could be a false positive roaming issue - Tower PLMN: {}, UE PLMN: {}",
                            self.likely_enb_plmn, self.likely_ue_plmn
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
        self.state = next_state;
    }

    // Sometimes an ENB can have multiple PLMNS
    fn format_plmn_list(&mut self, plmn_list: &PLMN_IdentityList) -> String {
        plmn_list
            .0
            .iter()
            .map(|info| self.plmn_identity_to_str(&info.plmn_identity))
            .collect::<Vec<_>>()
            .join(", ")
    }

    // PLMN is represented in two very different ways in the LTE spec so we need
    // two very different functions to decode them. I hate this.
    fn plmn_identity_to_str(&mut self, plmn: &PLMN_Identity) -> String {
        let mcc_digits: String = plmn.mcc
            .as_ref()
            .map(|mcc| mcc.0.iter()
                .map(|MCC_MNC_Digit(n)| n.to_string())
                .collect::<String>())
            .unwrap_or_default();
        
        let mnc_digits: String = plmn.mnc
            .0.iter()
            .map(|MCC_MNC_Digit(n)| n.to_string())
            .collect::<String>();
        
        format!("{}-{}", mcc_digits, mnc_digits)
    }

    fn plmn_vec_to_str(&mut self, bytes: &[u8]) -> String {
        let mcc_digit1 = bytes[0] & 0x0F;
        let mcc_digit2 = (bytes[0] >> 4) & 0x0F;
        let mcc_digit3 = bytes[1] & 0x0F;

        let mnc_digit1 = bytes[2] & 0x0F;
        let mnc_digit2 = (bytes[2] >> 4) & 0x0F;
        let mnc_digit3 = (bytes[1] >> 4) & 0x0F;

        let mcc = mcc_digit1 as u32 * 100 + mcc_digit2 as u32 * 10 + mcc_digit3 as u32;

        let mcc_str = format!("{:03}", mcc);
        let mnc_str = if mnc_digit3 == 0xF {
            format!("{:02}", mnc_digit1 * 10 + mnc_digit2)
        } else {
            format!(
                "{:03}",
                mnc_digit1 as u32 * 100 + mnc_digit2 as u32 * 10 + mnc_digit3 as u32
            )
        };

        format!("{}-{}", mcc_str, mnc_str)
    }

    fn extract_plmn(&mut self, old_tai: &Option<TAI>) -> String {
        match old_tai {
            Some(t) => self.plmn_vec_to_str(&t.plmn),
            None => "Unknown".to_string(),
        }
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
        4
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
    ) -> Option<Event> {
        // Set the enodeb plmn to the last sib1 we got, we should improve this once we have PCI data, this
        // is a naive approach.
        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(sch_msg) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(c1) = &sch_msg.message
            && let BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) = c1
        {
            let plmn = &sib1.cell_access_related_info.plmn_identity_list;
            self.likely_enb_plmn = self.format_plmn_list(plmn);

            return None;
        }

        if let InformationElement::LTE(inner) = ie {
            match &**inner {
                LteInformationElement::NAS(payload) => match payload {
                    NASMessage::EMMMessage(EMMMessage::EMMAttachRequest(request)) => {
                        if self.likely_ue_plmn == "Unknown" {
                            self.likely_ue_plmn = self.extract_plmn(&request.old_tai.inner);
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
                        self.transition(State::Disconnect, packet_num);
                        if reject.emm_cause.inner
                            == AttachRejectEMMCause::EPSServicesAndNonEPSServicesNotAllowed
                        {
                            self.flag = Some(Event {
                                event_type: EventType::Low,
                                message: "Identity requested without authentication but its likely a false positive unless your SIM card has an active plan".to_string(),
                            });
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
