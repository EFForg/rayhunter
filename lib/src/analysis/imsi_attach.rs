use crate::analysis::analyzer::{Analyzer, Event, EventType};
use crate::analysis::information_element::{InformationElement, LteInformationElement};
use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;
use pycrate_rs::nas::generated::emm::emm_attach_reject::EMMCauseEMMCause as AttachRejectEMMCause;
use pycrate_rs::nas::generated::emm::emm_detach_request_mt::EPSDetachTypeMTType;
use pycrate_rs::nas::generated::emm::emm_identity_request::IDTypeV;
use pycrate_rs::nas::generated::emm::emm_service_reject::EMMCauseEMMCause as ServiceRejectEMMCause;
use pycrate_rs::nas::generated::emm::emm_tracking_area_update_reject::EMMCauseEMMCause as TAURejectEMMCause;
use std::borrow::Cow;

pub struct ImsiAttachAnalyzer;

impl ImsiAttachAnalyzer {
    pub fn new() -> Self {
        ImsiAttachAnalyzer
    }

    fn is_imsi_exposing_nas(&self, nas_msg: &NASMessage) -> bool {
        match nas_msg {
            NASMessage::EMMMessage(emm_msg) => match emm_msg {
                EMMMessage::EMMIdentityRequest(req) => req.id_type.inner == IDTypeV::IMSI,

                EMMMessage::EMMTrackingAreaUpdateReject(reject) => {
                    matches!(
                        reject.emm_cause.inner,
                        TAURejectEMMCause::IllegalUE
                            | TAURejectEMMCause::IllegalME
                            | TAURejectEMMCause::EPSServicesNotAllowed
                            | TAURejectEMMCause::EPSServicesAndNonEPSServicesNotAllowed
                            | TAURejectEMMCause::TrackingAreaNotAllowed
                            | TAURejectEMMCause::EPSServicesNotAllowedInThisPLMN
                            | TAURejectEMMCause::RequestedServiceOptionNotAuthorizedInThisPLMN
                    )
                }

                EMMMessage::EMMAttachReject(reject) => {
                    matches!(
                        reject.emm_cause.inner,
                        AttachRejectEMMCause::IllegalUE
                            | AttachRejectEMMCause::IllegalME
                            | AttachRejectEMMCause::EPSServicesNotAllowed
                            | AttachRejectEMMCause::EPSServicesAndNonEPSServicesNotAllowed
                            | AttachRejectEMMCause::PLMNNotAllowed
                            | AttachRejectEMMCause::TrackingAreaNotAllowed
                            | AttachRejectEMMCause::RoamingNotAllowedInThisTrackingArea
                            | AttachRejectEMMCause::EPSServicesNotAllowedInThisPLMN
                            | AttachRejectEMMCause::NoSuitableCellsInTrackingArea
                            | AttachRejectEMMCause::RequestedServiceOptionNotAuthorizedInThisPLMN
                    )
                }

                EMMMessage::EMMDetachRequestMT(req) => {
                    // Original implementation: !(nas_eps.emm.detach_type_dl == 3)
                    req.eps_detach_type.inner.typ != EPSDetachTypeMTType::IMSIDetach
                }

                EMMMessage::EMMServiceReject(reject) => {
                    matches!(
                        reject.emm_cause.inner,
                        ServiceRejectEMMCause::IllegalUE
                            | ServiceRejectEMMCause::IllegalME
                            | ServiceRejectEMMCause::EPSServicesNotAllowed
                            | ServiceRejectEMMCause::UEIdentityCannotBeDerivedByTheNetwork
                            | ServiceRejectEMMCause::TrackingAreaNotAllowed
                            | ServiceRejectEMMCause::EPSServicesNotAllowedInThisPLMN
                            | ServiceRejectEMMCause::RequestedServiceOptionNotAuthorizedInThisPLMN
                    )
                }

                _ => false,
            },
            _ => false,
        }
    }
}

impl Analyzer for ImsiAttachAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        "IMSI-Exposed Message Detector".into()
    }

    fn get_description(&self) -> Cow<'_, str> {
        "Catches any and all messages that may expose IMSI. Can be quite noisy. \
        Based on the detection logic from the Marlin paper (\"They Know Where You Are: Tracking Mobile \
        Devices Using Cellular Infrastructure\"). Since we don't have traffic of many devices, we \
        cannot implement the original exposure ratio calculation, and naively trigger an event on \
        every exposure.".into()
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(&mut self, ie: &InformationElement, _packet_num: usize) -> Option<Event> {
        let lte_ie = match ie {
            InformationElement::LTE(inner) => inner,
            _ => return None,
        };

        match lte_ie.as_ref() {
            LteInformationElement::NAS(nas_msg) => {
                if self.is_imsi_exposing_nas(nas_msg) {
                    let message_type = match nas_msg {
                        NASMessage::EMMMessage(emm_msg) => match emm_msg {
                            EMMMessage::EMMIdentityRequest(_) => "EMM Identity Request (IMSI)",
                            EMMMessage::EMMTrackingAreaUpdateReject(_) => {
                                "EMM Tracking Area Update Reject"
                            }
                            EMMMessage::EMMAttachReject(_) => "EMM Attach Reject",
                            EMMMessage::EMMDetachRequestMT(_) => "EMM Detach Request (MT)",
                            EMMMessage::EMMServiceReject(_) => "EMM Service Reject",
                            _ => "Unknown EMM Message",
                        },
                        _ => "Unknown NAS Message",
                    };

                    Some(Event {
                        event_type: EventType::Informational,
                        message: format!(
                            "IMSI-exposing NAS message detected: {message_type}."
                        ),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}