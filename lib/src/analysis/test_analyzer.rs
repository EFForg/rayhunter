use std::borrow::Cow;

use telcom_parser::lte_rrc::{BCCH_DL_SCH_MessageType, BCCH_DL_SCH_MessageType_c1};

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};
use deku::bitvec::*;

pub struct TestAnalyzer {}

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

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<Event> {
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
                .load_be::<u32>();
            let tac = sib1
                .cell_access_related_info
                .tracking_area_code
                .0
                .as_bitslice()
                .load_be::<u32>();
            let plmn = &sib1.cell_access_related_info.plmn_identity_list.0;
            let mcc_string: String;

            // MCC are always 3 digits
            if let Some(mcc) = &plmn[0].plmn_identity.mcc {
                mcc_string = format!("{}{}{}", mcc.0[0].0, mcc.0[1].0, mcc.0[2].0);
            } else {
                mcc_string = "nomcc".to_string();
            }
            let mnc = &plmn[0].plmn_identity.mnc;
            let mnc_string: String;
            // MNC can be 2 or 3 digits
            if mnc.0.len() == 3 {
                mnc_string = format!("{}{}{}", mnc.0[0].0, mnc.0[1].0, mnc.0[2].0);
            } else if mnc.0.len() == 2 {
                mnc_string = format!("{}{}", mnc.0[0].0, mnc.0[1].0);
            } else {
                mnc_string = format!("{:?}", mnc.0);
            }

            return Some(Event {
                event_type: EventType::Low,
                message: format!(
                    "SIB1 received CID: {}, TAC: {}, PLMN: {}-{}",
                    cid, tac, mcc_string, mnc_string
                ),
            });
        }
        None
    }
}
