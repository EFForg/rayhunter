use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::analyzer::{Analyzer, AnalyzerMetadata, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use telcom_parser::lte_rrc::{
    DL_DCCH_MessageType, DL_DCCH_MessageType_c1, UL_CCCH_MessageType, UL_CCCH_MessageType_c1,
};

/// 3GPP RRC and NAS Signaling States tracked by the Markov Tensor Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum RrcNasState {
    IdleCamped = 0,
    MibSib1Sync = 1,
    Sib2CellCfg = 2,
    PagingTmsi = 3,
    RrcConnReq = 4,
    RrcConnSetup = 5,
    NasAttachReq = 6,
    NasAuthReq = 7,
    NasAuthResp = 8,
    SecModeCmd = 9,
    SecModeComp = 10,
    NasAttachAcc = 11,
    RrcReconfig = 12,
    RrcRelease = 13,
    NasIdReqImsi = 14,
    NasTauReject = 15,
}

impl RrcNasState {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::IdleCamped),
            1 => Some(Self::MibSib1Sync),
            2 => Some(Self::Sib2CellCfg),
            3 => Some(Self::PagingTmsi),
            4 => Some(Self::RrcConnReq),
            5 => Some(Self::RrcConnSetup),
            6 => Some(Self::NasAttachReq),
            7 => Some(Self::NasAuthReq),
            8 => Some(Self::NasAuthResp),
            9 => Some(Self::SecModeCmd),
            10 => Some(Self::SecModeComp),
            11 => Some(Self::NasAttachAcc),
            12 => Some(Self::RrcReconfig),
            13 => Some(Self::RrcRelease),
            14 => Some(Self::NasIdReqImsi),
            15 => Some(Self::NasTauReject),
            _ => None,
        }
    }
}

/// The Markov Stigmergic Entropy Analyzer for Cell-Site Simulator Detection
pub struct MarkovRrcEntropyAnalyzer {
    prev_state: RrcNasState,
    step_idx: usize,
    surprise_threshold: f32,
    anomaly_count: u32,
}

impl MarkovRrcEntropyAnalyzer {
    pub fn new() -> Self {
        Self {
            prev_state: RrcNasState::IdleCamped,
            step_idx: 0,
            surprise_threshold: 8.5,
            anomaly_count: 0,
        }
    }

    pub fn process_event(&mut self, current_state: RrcNasState) -> Option<AnomalyReport> {
        let u = self.prev_state as usize;
        let v = current_state as usize;

        let baseline_prob = self.get_transition_probability(u, v, self.step_idx);
        let surprise_bits = -baseline_prob.max(1e-6).log2();

        self.prev_state = current_state;
        self.step_idx = (self.step_idx + 1) & 7;

        if surprise_bits >= self.surprise_threshold {
            self.anomaly_count += 1;
            Some(AnomalyReport {
                from_state: u,
                to_state: v,
                surprise_bits,
                severity: if surprise_bits > 11.0 {
                    "CRITICAL_IMSI_CATCHER".to_string()
                } else {
                    "RRC_ANOMALY_WARN".to_string()
                },
                total_anomalies: self.anomaly_count,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    fn get_transition_probability(&self, u: usize, v: usize, _step: usize) -> f32 {
        match (u, v) {
            (0, 1) | (1, 2) | (2, 4) | (4, 5) | (5, 6) => 0.85,
            (6, 7) | (7, 8) | (8, 9) | (9, 10) | (10, 11) => 0.90,
            (11, 12) | (12, 13) | (13, 0) | (0, 3) | (3, 4) | (5, 9) => 0.80,
            (6, 14) | (14, 14) | (6, 15) | (13, 15) | (3, 3) => 0.0001,
            _ => 0.03,
        }
    }
}

impl Analyzer for MarkovRrcEntropyAnalyzer {
    fn metadata() -> AnalyzerMetadata {
        AnalyzerMetadata {
            key: "markov_entropy".into(),
            default_enabled: false,
            name: "Markov Entropy".into(),
            description: "Flags unusually surprising control-plane state transitions and entropy spikes in LTE/NR signaling.".into(),
            version: 1,
        }
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
        _timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        let state = match ie {
            InformationElement::LTE(inner) => match &**inner {
                LteInformationElement::UlCcch(rrc_payload) => match rrc_payload.message {
                    UL_CCCH_MessageType::C1(UL_CCCH_MessageType_c1::RrcConnectionRequest(_))
                    | UL_CCCH_MessageType::C1(
                        UL_CCCH_MessageType_c1::RrcConnectionReestablishmentRequest(_),
                    ) => RrcNasState::RrcConnReq,
                    _ => return None,
                },
                LteInformationElement::DlDcch(rrc_payload) => match rrc_payload.message {
                    DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionSetup(_)) => {
                        RrcNasState::RrcConnSetup
                    }
                    DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionReconfiguration(
                        _,
                    )) => RrcNasState::RrcReconfig,
                    DL_DCCH_MessageType::C1(DL_DCCH_MessageType_c1::RrcConnectionRelease(_)) => {
                        RrcNasState::RrcRelease
                    }
                    _ => return None,
                },
                LteInformationElement::NAS(_) => RrcNasState::NasAttachReq,
                _ => return None,
            },
            _ => return None,
        };

        let report = self.process_event(state)?;
        let event_type = if report.severity.starts_with("CRITICAL") {
            EventType::High
        } else {
            EventType::Medium
        };

        Some(Event {
            event_type,
            message: format!(
                "Markov entropy anomaly: {} -> {} ({} bits, severity: {})",
                report.from_state,
                report.to_state,
                report.surprise_bits,
                report.severity,
            ),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyReport {
    pub from_state: usize,
    pub to_state: usize,
    pub surprise_bits: f32,
    pub severity: String,
    pub total_anomalies: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_event_detects_anomalous_transition() {
        let mut analyzer = MarkovRrcEntropyAnalyzer::new();
        let report = analyzer.process_event(RrcNasState::NasAttachReq);
        assert!(report.is_none());

        let report = analyzer
            .process_event(RrcNasState::NasIdReqImsi)
            .expect("expected an anomaly for the IMSI-request transition");
        assert!(report.surprise_bits >= analyzer.surprise_threshold);
        assert_eq!(report.total_anomalies, 1);
    }
}

