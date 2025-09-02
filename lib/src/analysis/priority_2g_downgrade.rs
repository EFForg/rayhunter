use std::borrow::Cow;

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use telcom_parser::lte_rrc::{
    BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1, CellReselectionPriority,
    SystemInformation_r8_IEsSib_TypeAndInfo, SystemInformation_r8_IEsSib_TypeAndInfo_Entry,
    SystemInformationBlockType7, SystemInformationCriticalExtensions,
};

/// Based on heuristic T7 from Shinjo Park's "Why We Cannot Win".
pub struct LteSib6And7DowngradeAnalyzer {}

impl LteSib6And7DowngradeAnalyzer {
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
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<super::analyzer::Event> {
        let sibs = &self.unpack_system_information(ie)?.0;
        for sib in sibs {
            match sib {
                SystemInformation_r8_IEsSib_TypeAndInfo_Entry::Sib6(sib6) => {
                    if let Some(carrier_info_list) = sib6.carrier_freq_list_utra_fdd.as_ref() {
                        for carrier_info in &carrier_info_list.0 {
                            if let Some(CellReselectionPriority(p)) =
                                carrier_info.cell_reselection_priority
                                && p == 0
                            {
                                return Some(Event {
                                    event_type: EventType::High,
                                    message:
                                        "LTE cell advertised a 3G cell for priority 0 reselection"
                                            .to_string(),
                                });
                            }
                        }
                    }
                    if let Some(carrier_info_list) = sib6.carrier_freq_list_utra_tdd.as_ref() {
                        for carrier_info in &carrier_info_list.0 {
                            if let Some(CellReselectionPriority(p)) =
                                carrier_info.cell_reselection_priority
                                && p == 0
                            {
                                return Some(Event {
                                    event_type: EventType::High,
                                    message:
                                        "LTE cell advertised a 3G cell for priority 0 reselection"
                                            .to_string(),
                                });
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
                            && p == 0
                        {
                            return Some(Event {
                                event_type: EventType::High,
                                message: "LTE cell advertised a 2G cell for priority 0 reselection"
                                    .to_string(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}
