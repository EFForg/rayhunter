use chrono::{DateTime, FixedOffset};
use std::collections::BTreeSet;

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;

use super::analyzer::{Analyzer, AnalyzerMetadata, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use crate::plmn::{PACKED_BCD_LEN, decode_packed_bcd};
use log::{debug, error};

use pycrate_rs::nas::generated::emm::emm_attach_reject::EMMCauseEMMCause as AttachRejectEMMCause;
use pycrate_rs::nas::generated::emm::emm_attach_request::TAI;
use pycrate_rs::nas::generated::emm::emm_identity_request::IDTypeV;
use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};
use telcom_parser::lte_rrc::{
    DL_DCCH_MessageType, DL_DCCH_MessageType_c1, DLInformationTransfer,
    DLInformationTransfer_r8_IEsDedicatedInfoType,
    DLInformationTransfer_r15_IEsDedicatedInfoType_r15, DLInformationTransferCriticalExtensions,
    DLInformationTransferCriticalExtensions_c1, DedicatedInfoNAS, UL_CCCH_MessageType,
    UL_CCCH_MessageType_c1, UL_DCCH_MessageType, UL_DCCH_MessageType_c1, ULInformationTransfer,
    ULInformationTransfer_r8_IEsDedicatedInfoType,
    ULInformationTransfer_r16_IEsDedicatedInfoType_r16, ULInformationTransferCriticalExtensions,
    ULInformationTransferCriticalExtensions_c1,
};
use telcom_parser::lte_rrc::{MCC_MNC_Digit, PLMN_Identity, PLMN_IdentityList};

const TIMEOUT_THRESHHOLD: usize = 50;

const OTA_COPY_MAX_AGE: chrono::TimeDelta = chrono::TimeDelta::milliseconds(50);

/// For NAS message with r8 and r15 message layouts, return security header
/// (high nibble of first byte) if mobility mgmt.
fn dl_nas_security_header(transfer: &DLInformationTransfer) -> Option<u8> {
    let DLInformationTransferCriticalExtensions::C1(c1) = &transfer.critical_extensions else {
        return None;
    };
    let nas = match c1 {
        DLInformationTransferCriticalExtensions_c1::DlInformationTransfer_r8(ies) => {
            match &ies.dedicated_info_type {
                DLInformationTransfer_r8_IEsDedicatedInfoType::DedicatedInfoNAS(nas) => nas,
                _ => return None,
            }
        }
        DLInformationTransferCriticalExtensions_c1::DlInformationTransfer_r15(ies) => {
            match &ies.dedicated_info_type_r15 {
                Some(DLInformationTransfer_r15_IEsDedicatedInfoType_r15::DedicatedInfoNAS(nas)) => {
                    nas
                }
                _ => return None,
            }
        }
        _ => return None,
    };
    nas_security_header(nas)
}

fn ul_nas_security_header(transfer: &ULInformationTransfer) -> Option<u8> {
    let ULInformationTransferCriticalExtensions::C1(c1) = &transfer.critical_extensions else {
        return None;
    };
    let nas = match c1 {
        ULInformationTransferCriticalExtensions_c1::UlInformationTransfer_r8(ies) => {
            match &ies.dedicated_info_type {
                ULInformationTransfer_r8_IEsDedicatedInfoType::DedicatedInfoNAS(nas) => nas,
                _ => return None,
            }
        }
        ULInformationTransferCriticalExtensions_c1::UlInformationTransfer_r16(ies) => {
            match &ies.dedicated_info_type_r16 {
                Some(ULInformationTransfer_r16_IEsDedicatedInfoType_r16::DedicatedInfoNAS_r16(
                    nas,
                )) => nas,
                _ => return None,
            }
        }
        _ => return None,
    };
    nas_security_header(nas)
}

fn nas_security_header(nas: &DedicatedInfoNAS) -> Option<u8> {
    let first = *nas.0.first()?;
    ((first & 0x0F) == 0x07).then_some(first >> 4)
}

fn is_integrity_protected(security_header: u8) -> bool {
    matches!(security_header, 1..=4)
}

fn is_ciphered(security_header: u8) -> bool {
    matches!(security_header, 2 | 4)
}

fn within_ota_window(earlier: DateTime<FixedOffset>, later: DateTime<FixedOffset>) -> bool {
    let age = later - earlier;
    age >= chrono::TimeDelta::zero() && age <= OTA_COPY_MAX_AGE
}

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
    last_dl_nas_security_header: Option<(u8, DateTime<FixedOffset>)>,
    imeisv_requested: bool,
    imeisv_response_at: Option<DateTime<FixedOffset>>,
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
            last_dl_nas_security_header: None,
            imeisv_requested: false,
            imeisv_response_at: None,
        }
    }

    /// Check security header of over-the-air copy.
    fn ota_security_header(&mut self, timestamp: DateTime<FixedOffset>) -> Option<u8> {
        let (header, seen_at) = self.last_dl_nas_security_header.take()?;
        within_ota_window(seen_at, timestamp).then_some(header)
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
    fn metadata() -> AnalyzerMetadata {
        AnalyzerMetadata {
            key: "imsi_requested".into(),
            default_enabled: true,
            name: "IMSI Requested".into(),
            description: "Tests whether the ME sends an Identity Request NAS message without either an associated attach request or auth accept message".into(),
            version: 6,
        }
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        packet_num: usize,
        timestamp: DateTime<FixedOffset>,
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
                LteInformationElement::NAS(payload) => {
                    self.imeisv_response_at = None;
                    let ota_header = self.ota_security_header(timestamp);
                    match payload {
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
                        NASMessage::EMMMessage(EMMMessage::EMMIdentityRequest(request)) => {
                            self.imeisv_requested =
                                matches!(request.id_type.inner, IDTypeV::IMEISV);
                            if self.imeisv_requested
                                && let Some(header) = ota_header
                            {
                                let (event_type, message) = if is_ciphered(header) {
                                    (
                                        EventType::Informational,
                                        "IMEISV requested with NAS integrity protection and ciphering, normal on some networks",
                                    )
                                } else if is_integrity_protected(header) {
                                    (
                                        EventType::Low,
                                        "IMEISV requested with NAS integrity protection but without ciphering",
                                    )
                                } else {
                                    (
                                        EventType::High,
                                        "IMEISV requested without NAS integrity protection",
                                    )
                                };
                                self.flag = Some(Event {
                                    event_type,
                                    message: message.to_string(),
                                });
                            } else {
                                self.transition(State::IdentityRequest, packet_num);
                            }
                        }
                        NASMessage::EMMMessage(EMMMessage::EMMIdentityResponse(_)) => {
                            if std::mem::take(&mut self.imeisv_requested) {
                                self.imeisv_response_at = Some(timestamp);
                            }
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
                    }
                }

                LteInformationElement::UlCcch(rrc_payload) => match rrc_payload.message {
                    UL_CCCH_MessageType::C1(UL_CCCH_MessageType_c1::RrcConnectionRequest(_))
                    | UL_CCCH_MessageType::C1(
                        UL_CCCH_MessageType_c1::RrcConnectionReestablishmentRequest(_),
                    ) => {
                        self.transition(State::AttachRequest, packet_num);
                    }
                    _ => {}
                },

                LteInformationElement::UlDcch(rrc_payload) => {
                    if let UL_DCCH_MessageType::C1(UL_DCCH_MessageType_c1::UlInformationTransfer(
                        transfer,
                    )) = &rrc_payload.message
                        && let Some(header) = ul_nas_security_header(transfer)
                        && let Some(sent_at) = self.imeisv_response_at.take()
                        && within_ota_window(sent_at, timestamp)
                        && !is_ciphered(header)
                    {
                        self.flag = Some(Event {
                            event_type: EventType::High,
                            message: "IMEISV sent without NAS ciphering".to_string(),
                        });
                    }
                }

                LteInformationElement::DlDcch(rrc_payload) => {
                    if let DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionRelease(
                        _,
                    )) = rrc_payload.message
                    {
                        self.transition(State::Disconnect, packet_num);
                        self.imeisv_requested = false;
                        self.imeisv_response_at = None;
                    }
                    // Grab header.
                    if let DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::DlInformationTransfer(
                        transfer,
                    )) = &rrc_payload.message
                        && let Some(header) = dl_nas_security_header(transfer)
                    {
                        self.last_dl_nas_security_header = Some((header, timestamp));
                    }
                }
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
                self.flag.get_or_insert(Event {
                    event_type: EventType::Informational {},
                    message: "Identity request happened without auth request followup".to_string(),
                });
                self.timeout_counter = 0;
            }
        }

        self.flag.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, LteNasSubtype, LteRrcSubtype};

    const AUTH_RESPONSE: &[u8] = &[0x07, 0x53, 0x08, 0, 0, 0, 0, 0, 0, 0, 0];
    const ID_REQUEST_IMSI: &[u8] = &[0x07, 0x55, 0x01];
    const ID_REQUEST_IMEI: &[u8] = &[0x07, 0x55, 0x02];
    const ID_REQUEST_IMEISV: &[u8] = &[0x07, 0x55, 0x03];
    const DL_PROTECTED: &[u8] = &[
        0x0c, 0x00, 0x49, 0x3b, 0xce, 0x6c, 0xea, 0x68, 0x0d, 0xaa, 0x2d, 0xc0,
    ];
    const DL_PLAIN: &[u8] = &[0x08, 0x00, 0x18, 0x3a, 0xa8, 0x18];
    const DL_INTEGRITY_ONLY: &[u8] = &[
        0x0c, 0x00, 0x48, 0xbb, 0xce, 0x6c, 0xea, 0x68, 0x0d, 0xaa, 0x2d, 0xc0,
    ];
    const EMM_INFORMATION: &[u8] = &[0x07, 0x61];
    const ID_RESPONSE: &[u8] = &[0x07, 0x56, 0x09, 0x33, 0, 0, 0, 0, 0, 0, 0, 0xf0];
    const UL_PROTECTED: &[u8] = &[
        0x48, 0x02, 0x44, 0xeb, 0x45, 0x0e, 0x13, 0xe0, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const UL_INTEGRITY_ONLY: &[u8] = &[
        0x48, 0x02, 0x42, 0xeb, 0x45, 0x0e, 0x13, 0xe0, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const UL_PLAIN: &[u8] = &[
        0x48, 0x02, 0x40, 0xea, 0xc1, 0x20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    fn time() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2026-11-08T13:12:11+00:00").unwrap()
    }

    fn nas(payload: &[u8]) -> InformationElement {
        InformationElement::try_from(&GsmtapMessage {
            header: GsmtapHeader::new(GsmtapType::LteNas(LteNasSubtype::Plain)),
            payload: payload.to_vec(),
        })
        .expect("failed to parse test NAS message")
    }

    fn dl_dcch(payload: &[u8]) -> InformationElement {
        InformationElement::try_from(&GsmtapMessage {
            header: GsmtapHeader::new(GsmtapType::LteRrc(LteRrcSubtype::DlDcch)),
            payload: payload.to_vec(),
        })
        .expect("failed to parse test RRC message")
    }

    fn ul_dcch(payload: &[u8]) -> InformationElement {
        InformationElement::try_from(&GsmtapMessage {
            header: GsmtapHeader::new(GsmtapType::LteRrc(LteRrcSubtype::UlDcch)),
            payload: payload.to_vec(),
        })
        .expect("failed to parse test RRC message")
    }

    fn run(messages: &[InformationElement]) -> Vec<Event> {
        let mut analyzer = ImsiRequestedAnalyzer::default();
        messages
            .iter()
            .enumerate()
            .filter_map(|(i, ie)| analyzer.analyze_information_element(ie, i, time()))
            .collect()
    }

    fn run_timed(messages: &[(InformationElement, i64)]) -> Vec<Event> {
        let mut analyzer = ImsiRequestedAnalyzer::default();
        messages
            .iter()
            .enumerate()
            .filter_map(|(i, (ie, ms))| {
                let t = time() + chrono::TimeDelta::milliseconds(*ms);
                analyzer.analyze_information_element(ie, i, t)
            })
            .collect()
    }

    fn highs(events: &[Event]) -> usize {
        events
            .iter()
            .filter(|e| e.event_type == EventType::High)
            .count()
    }

    #[test]
    fn protected_imeisv_request_after_auth_is_informational() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_PROTECTED),
            nas(ID_REQUEST_IMEISV),
        ]);
        assert_eq!(highs(&events), 0);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Informational);
    }

    #[test]
    fn plain_imeisv_request_after_auth_is_high() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_PLAIN),
            nas(ID_REQUEST_IMEISV),
        ]);
        assert_eq!(highs(&events), 1);
        assert_eq!(
            events[0].message,
            "IMEISV requested without NAS integrity protection"
        );
    }

    #[test]
    fn plain_imeisv_request_before_auth_is_high() {
        let events = run(&[dl_dcch(DL_PLAIN), nas(ID_REQUEST_IMEISV)]);
        assert_eq!(highs(&events), 1);
        assert_eq!(
            events[0].message,
            "IMEISV requested without NAS integrity protection"
        );
    }

    #[test]
    fn imeisv_request_after_auth_without_ota_copy_keeps_old_alert() {
        let events = run(&[nas(AUTH_RESPONSE), nas(ID_REQUEST_IMEISV)]);
        assert_eq!(highs(&events), 1);
        assert_eq!(events[0].message, "Identity requested after auth request");
    }

    #[test]
    fn integrity_only_imeisv_request_is_low() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_INTEGRITY_ONLY),
            nas(ID_REQUEST_IMEISV),
        ]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Low);
    }

    #[test]
    fn imsi_and_imei_requests_after_auth_are_still_high_even_if_protected() {
        for request in [ID_REQUEST_IMSI, ID_REQUEST_IMEI] {
            let events = run(&[nas(AUTH_RESPONSE), dl_dcch(DL_PROTECTED), nas(request)]);
            assert_eq!(highs(&events), 1);
            assert_eq!(events[0].message, "Identity requested after auth request");
        }
    }

    #[test]
    fn ciphered_imeisv_response_raises_nothing_more() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_PROTECTED),
            nas(ID_REQUEST_IMEISV),
            nas(ID_RESPONSE),
            ul_dcch(UL_PROTECTED),
        ]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Informational);
    }

    #[test]
    fn unciphered_imeisv_response_is_high() {
        for ul in [UL_INTEGRITY_ONLY, UL_PLAIN] {
            let events = run(&[
                nas(AUTH_RESPONSE),
                dl_dcch(DL_PROTECTED),
                nas(ID_REQUEST_IMEISV),
                nas(ID_RESPONSE),
                ul_dcch(ul),
            ]);
            assert_eq!(highs(&events), 1);
            assert_eq!(events.len(), 2);
            assert_eq!(events[1].message, "IMEISV sent without NAS ciphering");
        }
    }

    #[test]
    fn response_to_imsi_request_is_not_checked() {
        let events = run(&[nas(ID_REQUEST_IMSI), nas(ID_RESPONSE), ul_dcch(UL_PLAIN)]);
        assert!(events.is_empty());
    }

    #[test]
    fn imsi_request_after_auth_is_high() {
        let events = run(&[nas(AUTH_RESPONSE), nas(ID_REQUEST_IMSI)]);
        assert_eq!(highs(&events), 1);
        assert_eq!(events[0].message, "Identity requested after auth request");
    }

    #[test]
    fn stale_ota_copy_is_not_trusted() {
        let events = run_timed(&[
            (nas(AUTH_RESPONSE), 0),
            (dl_dcch(DL_PROTECTED), 0),
            (
                nas(ID_REQUEST_IMEISV),
                OTA_COPY_MAX_AGE.num_milliseconds() + 1,
            ),
        ]);
        assert_eq!(highs(&events), 1);
        assert_eq!(events[0].message, "Identity requested after auth request");
    }

    #[test]
    fn stale_imeisv_response_ota_copy_is_ignored() {
        let events = run_timed(&[
            (nas(AUTH_RESPONSE), 0),
            (dl_dcch(DL_PROTECTED), 0),
            (nas(ID_REQUEST_IMEISV), 0),
            (nas(ID_RESPONSE), 0),
            (ul_dcch(UL_PLAIN), OTA_COPY_MAX_AGE.num_milliseconds() + 1),
        ]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Informational);
    }

    #[test]
    fn ota_copy_only_vouches_for_the_next_nas_message() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_PROTECTED),
            nas(EMM_INFORMATION),
            nas(ID_REQUEST_IMEISV),
        ]);
        assert_eq!(highs(&events), 1);
        assert_eq!(events[0].message, "Identity requested after auth request");
    }

    #[test]
    fn imeisv_response_slot_is_cleared_by_the_next_nas_message() {
        let events = run(&[
            nas(AUTH_RESPONSE),
            dl_dcch(DL_PROTECTED),
            nas(ID_REQUEST_IMEISV),
            nas(ID_RESPONSE),
            nas(EMM_INFORMATION),
            ul_dcch(UL_PLAIN),
        ]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::Informational);
    }

    #[test]
    fn timeout_does_not_erase_plain_imeisv_request_high() {
        let mut messages = vec![nas(ID_REQUEST_IMSI)];
        messages.resize_with(TIMEOUT_THRESHHOLD - 2, || nas(EMM_INFORMATION));
        messages.push(dl_dcch(DL_PLAIN));
        messages.push(nas(ID_REQUEST_IMEISV));
        let events = run(&messages);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::High);
    }
}
