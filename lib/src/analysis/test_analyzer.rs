use std::borrow::Cow;

use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use deku::bitvec::*;

pub struct TestAnalyzer {
    packet_num: usize,
}

impl Default for TestAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TestAnalyzer {
    pub fn new() -> Self {
        Self { packet_num: 0 }
    }
}

impl Analyzer for TestAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Test Analyzer")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from(
            "This is an analyzer which can be used to test that your rayhunter is working. It will generate an alert for every SIB1 message (a beacon from the cell tower) that it sees. Do not leave this on when you are hunting or it will be very noisy.",
        )
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event> {
        self.packet_num += 1;

        if let InformationElement::LTE(lte_ie) = ie
            && let LteInformationElement::BcchDlSch(sch_msg) = &**lte_ie
            && let BCCH_DL_SCH_MessageType::C1(c1) = &sch_msg.message
            && let BCCH_DL_SCH_MessageType_c1::SystemInformationBlockType1(sib1) = c1
        {
            let cid = sib1
                .cell_access_related_info
                .cell_identity
                .0
                .as_bitslice()
                .load::<u32>();
            let plmn = &sib1.cell_access_related_info.plmn_identity_list.0;
            let mcc_string: String;

            if let Some(mcc) = &plmn[0].plmn_identity.mcc {
                mcc_string = format!("{}{}{}", mcc.0[0].0, mcc.0[1].0, mcc.0[2].0);
            } else {
                mcc_string = "nomcc".to_string();
            }
            let mnc = &plmn[0].plmn_identity.mnc;
            let mnc_string: String = format!("{}{}{}", mnc.0[0].0, mnc.0[1].0, mnc.0[2].0);

            return Some(Event {
                event_type: EventType::Low,
                message: format!(
                    "SIB1 received (packet {}) CID: {}, PLMN: {}-{}",
                    self.packet_num, cid, mcc_string, mnc_string
                ),
            });
        }
        None
    }
}
