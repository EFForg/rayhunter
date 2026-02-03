use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use log::debug;
use telcom_parser::lte_rrc::{
    BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1, CellReselectionPriority,
    SystemInformation_r8_IEsSib_TypeAndInfo, SystemInformation_r8_IEsSib_TypeAndInfo_Entry,
    SystemInformationBlockType7, SystemInformationCriticalExtensions,
};

/// Based on heuristic T7 from Shinjo Park's "Why We Cannot Win".
pub struct LteSib6And7DowngradeAnalyzer {
    lte_priority: Option<u8>,
    legacy_priority: Option<u8>,
}
impl Default for LteSib6And7DowngradeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl LteSib6And7DowngradeAnalyzer {
    pub fn new() -> Self {
        Self {
            lte_priority: None,
            legacy_priority: None,
        }
    }

    fn unpack_system_information<'a>(
        &self,
        ie: &'a InformationElement,
    ) -> Option<&'a SystemInformation_r8_IEsSib_TypeAndInfo> {
        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(bcch_dl_sch_message) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(BCCH_DL_SCH_MessageType_c1::SystemInformation(
                system_information,
            )) = &bcch_dl_sch_message.message
            && let SystemInformationCriticalExtensions::SystemInformation_r8(sib) =
                &system_information.critical_extensions
        {
            return Some(&sib.sib_type_and_info);
        }
        None
    }
}

// TODO: keep track of SIB state to compare LTE reselection blocks w/ 2g/3g ones
impl Analyzer for LteSib6And7DowngradeAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("LTE SIB 6/7 Downgrade")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "Tests for LTE cells broadcasting a SIB type 6 and 7 which include 2G/3G frequencies with higher priorities.",
        )
    }

    fn get_version(&self) -> u32 {
        2
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<super::analyzer::Event> {
        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(sch_msg) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(c1) = &sch_msg.message
            && let BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(_) = c1
        {
            debug!("encountered sib1 at {_packet_num}");
            let flag = if self.legacy_priority > self.lte_priority {
                if self.lte_priority.is_none() {
                    Some(Event {
                        event_type: EventType::Informational,
                        message:
                            "LTE cell advertised a legacy (3G/2G) neighbors but no LTE neighbors"
                                .to_string(),
                    })
                } else {
                    Some(Event {
                        event_type: EventType::High,
                        message:
                            format!("LTE cell advertised a legacy (3G/2G) cell for priority {:?} reselection over LTE neighbors at priority {:?}", self.legacy_priority?, self.lte_priority)
                                .to_string(),
                    })
                }
            } else {
                None
            };
            debug!(
                "flag is {flag:?} because lte priority is {:?} and legacy priority is {:?} ",
                self.lte_priority, self.legacy_priority
            );
            self.lte_priority = None;
            self.legacy_priority = None;
            debug!("reset priority to 0 due to new sib1 at {_packet_num}");
            return flag;
        }

        let sibs = &self.unpack_system_information(ie)?.0;
        for sib in sibs {
            match sib {
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib3(sib3) => {
                    let res_p: u8 = sib3
                        .cell_reselection_serving_freq_info
                        .cell_reselection_priority
                        .0;
                    if Some(res_p) > self.lte_priority {
                        self.lte_priority = Some(res_p);
                        debug!("set priority {} due to sib3 (frame {})", res_p, _packet_num);
                    }
                }
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib5(sib5) => {
                    let carrier_freq_list = &sib5.inter_freq_carrier_freq_list;
                    for carrier_freq in &carrier_freq_list.0 {
                        if let Some(res_p) = &carrier_freq.cell_reselection_priority {
                            let pri: u8 = res_p.0;
                            if Some(pri) > self.lte_priority {
                                self.lte_priority = Some(pri);
                                debug!("set priority {} due to sib5 (frame {})", pri, _packet_num);
                            }
                        }
                    }
                }
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib6(sib6) => {
                    if let Some(carrier_info_list) = sib6.carrier_freq_list_utra_fdd.as_ref() {
                        for carrier_info in &carrier_info_list.0 {
                            if let Some(CellReselectionPriority(p)) =
                                carrier_info.cell_reselection_priority
                                && Some(p) > self.legacy_priority
                            {
                                self.legacy_priority = Some(p);
                                debug!(
                                    "set legacy priority {} due to sib6 (frame {})",
                                    p, _packet_num
                                );
                            }
                        }
                    }
                    if let Some(carrier_info_list) = sib6.carrier_freq_list_utra_tdd.as_ref() {
                        for carrier_info in &carrier_info_list.0 {
                            if let Some(CellReselectionPriority(p)) =
                                carrier_info.cell_reselection_priority
                                && Some(p) > self.legacy_priority
                            {
                                self.legacy_priority = Some(p);
                                debug!(
                                    "set legacy priority {} due to sib6 (frame {})",
                                    p, _packet_num
                                );
                            }
                        }
                    }
                }
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib7(
                    SystemInformationBlockType7 {
                        carrier_freqs_info_list: Some(carrier_info_list),
                        ..
                    },
                ) => {
                    for carrier_info in &carrier_info_list.0 {
                        if let Some(CellReselectionPriority(p)) =
                            carrier_info.common_info.cell_reselection_priority
                            && Some(p) > self.legacy_priority
                        {
                            self.legacy_priority = p.into();
                            debug!(
                                "set legacy priority {} due to sib7 (frame {})",
                                p, _packet_num
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}
