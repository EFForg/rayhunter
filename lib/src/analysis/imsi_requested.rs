use std::borrow::Cow;

use pycrate_rs::nas::NASMessage;
use pycrate_rs::nas::emm::EMMMessage;

use super::analyzer::{Analyzer, Event, EventType, Severity};
use super::information_element::{InformationElement, LteInformationElement};
use log::debug;

use telcom_parser::lte_rrc::{DL_DCCH_MessageType, DL_DCCH_MessageType_c1, UL_CCCH_MessageType, UL_CCCH_MessageType_c1};

const TIMEOUT_THRESHHOLD: usize = 40;

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
    flag: Option<Event>,
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
           
            // Reset timeout on successful auth 
            (_, State::AuthAccept) => {
                debug!("reset timeout counter at {} due to auth accept (frame {})", self.timeout_counter, self.packet_num);
                self.timeout_counter = 0;
            }

            // Unexpected IMSI without AttachRequest
            (current, State::IdentityRequest) if *current != State::AttachRequest => {
                self.flag = Some( Event {
                    event_type: EventType::QualitativeWarning { severity: Severity::High },
                    message: format!("Identity requested without Attach Request (frame {})", self.packet_num).to_string(),
                });
            }

            // IMSI to Disconnect without AuthAccept
            (State::IdentityRequest, State::Disconnect) => {
                self.flag = Some( Event {
                    event_type: EventType::Informational,
                    message: format!("Disconnected after Identity Request without Auth Accept (frame {})", self.packet_num).to_string(),
                });
                println!("reset timeout counter at {}/{} due to disconnect", self.packet_num, self.timeout_counter);
                self.timeout_counter = 0;
            }

            // All other transitions proceeed
            _ => {
                debug!("Transition from {:?} to {:?} at {}", self.state, next_state, self.packet_num);
            }
        }
        self.state = next_state;
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

        match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::NAS(payload) => {
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
                        _ => {}
                    }
                },

                LteInformationElement::UlCcch(rrc_payload) => {
                    match rrc_payload.message {
                        UL_CCCH_MessageType::C1(UL_CCCH_MessageType_c1::RrcConnectionRequest(_))
                        | UL_CCCH_MessageType::C1( UL_CCCH_MessageType_c1::RrcConnectionReestablishmentRequest(_),) => {
                            self.transition(State::AttachRequest);
                        },
                        _ => {}
                    }
                }

                LteInformationElement::DlDcch(rrc_payload) => {
                    match rrc_payload.message {
                        DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionRelease(_)) => {
                            self.transition(State::Disconnect)
                        },
                        _ => {}
                    }
                }
                _ => {},
            },
            _ => {},
        };

        if self.state == State::IdentityRequest {
            self.timeout_counter += 1;
            debug!("timeout: counter {}, packet: {}", self.timeout_counter, self.packet_num);
            if self.timeout_counter >= TIMEOUT_THRESHHOLD {
                self.flag = Some(Event {
                    event_type: EventType::Informational {},
                    message: format!("Identity request happened without auth request followup (frame {})", self.packet_num).to_string(),
                });
                self.timeout_counter = 0;
            }

        }

        return self.flag.take();
    }
}
