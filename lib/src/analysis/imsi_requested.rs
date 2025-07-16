use std::borrow::Cow;

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};

use telcom_parser::lte_rrc::{UL_CCCH_MessageType, UL_CCCH_MessageType_c1};

const PACKET_THRESHHOLD: usize = 20;

#[derive(PartialEq, Debug)]
pub enum State {
    Unattached,
    AttachRequest,
    IdentityRequest,
    AuthAccept,
    Disconnect,
}

pub struct ImsiRequestedAnalyzer {
    packet_num: usize,
    state: State,
    timeout_counter: usize,
    flag: Option<(bool /*true=warning, false=info */, Severity, String)>,
}

impl Default for ImsiRequestedAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ImsiRequestedAnalyzer {
    pub fn new() -> Self {
        Self {
            packet_num: 0,
            state: State::Unattached,
            timeout_counter: 0,
            flag: None,
        }
    }

    fn transition(&mut self, next_state: State) {
        match (&self.state, &next_state) {
            /*
            // Valid transitions
            (State::AttachRequest, State::IdentityRequest) |
            (State::IdentityRequest, State::AuthAccept) |
            (State::Disconnect, State::AttachRequest) |
            (State::AttachRequest, State::Disconnect) => {
                self.state = next_state;
            }
            */
            // Reset on successful auth
            (_, State::AuthAccept) | (State::AuthAccept, State::Disconnect) => {
                self.state = next_state;
                self.timeout_counter = 0;
            }

            // Unexpected IMSI without AttachRequest
            (_, State::IdentityRequest) if self.state != State::AttachRequest => {
                self.flag = Some((
                    true,
                    Severity::High,
                    "Identity requested without Attach Request".to_string(),
                ));
                self.state = next_state;
            }

            // IMSI to Disconnect without AuthAccept
            (State::IdentityRequest, State::Disconnect) => {
                self.flag = Some((
                    false,
                    Severity::Low,
                    "Disconnected after Identity Request without Auth Accept".to_string(),
                ));
                self.state = next_state;
            }

            // All other transitions proceeed
            _ => {
                //println!("Transition from {:?} to {:?}", self.state, next_state);
                self.state = next_state;
            }
        }
    }
}

impl Analyzer for ImsiRequestedAnalyzer {
    fn get_name(&self) -> Cow<str> {
        Cow::from("Identity (IMSI or IMEI) requested in suspicious manner")
    }

    fn get_description(&self) -> Cow<str> {
        Cow::from(
            "Tests whether the ME sends an Identity Request NAS message without either an associated attach request or auth accept message",
        )
    }

    fn get_version(&self) -> u32 {
        2
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;
        let maybe_payload = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::NAS(payload) => Some(payload),
                _ => None,
            },
            _ => None,
        };

        let maybe_rrc_payload = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::UlCcch(rrc_payload) => Some(rrc_payload),
                _ => None,
            },
            _ => None,
        };
        if let Some(payload) = maybe_payload {
            match payload {
                NASMessage::EMMMessage(EMMMessage::EMMExtServiceRequest(_))
                | NASMessage::EMMMessage(EMMMessage::EMMAttachRequest(_)) => {
                    self.transition(State::AttachRequest);
                }
                NASMessage::EMMMessage(EMMMessage::EMMIdentityRequest(_)) => {
                    self.transition(State::IdentityRequest);
                }
                NASMessage::EMMMessage(EMMMessage::EMMAuthenticationResponse(_)) => {
                    self.transition(State::AuthAccept);
                }
                NASMessage::EMMMessage(EMMMessage::EMMServiceReject(_)) => {
                    self.transition(State::Disconnect);
                }
                NASMessage::EMMMessage(EMMMessage::EMMAttachReject(_)) => {
                    self.transition(State::Disconnect);
                }
                NASMessage::EMMMessage(EMMMessage::EMMTrackingAreaUpdateReject(_)) => {
                    self.transition(State::Disconnect);
                }
                _ => {
                    return None;
                }
            }
        }

        if let Some(rrc_payload) = maybe_rrc_payload {
            match rrc_payload.message {
                UL_CCCH_MessageType::C1(UL_CCCH_MessageType_c1::RrcConnectionRequest(_))
                | UL_CCCH_MessageType::C1(
                    UL_CCCH_MessageType_c1::RrcConnectionReestablishmentRequest(_),
                ) => {
                    self.transition(State::AttachRequest);
                }
                _ => {
                    return None;
                }
            }
        }

        if self.state == State::IdentityRequest {
            self.timeout_counter += 1;
            if self.timeout_counter > PACKET_THRESHHOLD {
                self.flag = Some((
                    true,
                    Severity::High,
                    "Identity request happened without auth request followup".to_string(),
                ));
            }
        }

        if let Some(flag) = self.flag.clone() {
            let (warning, severity, message) = flag;
            self.flag = None; // Clear the flag
            if warning {
                return Some(Event {
                    event_type: EventType::QualitativeWarning { severity: severity },
                    message: message,
                });
            } else {
                return Some(Event {
                    event_type: EventType::Informational,
                    message: message,
                });
            }
        } else {
            return None;
        }
    }
}
