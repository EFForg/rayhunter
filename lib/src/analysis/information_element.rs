//! The term "information element" is used by 3GPP to describe "structural
//! elements containing single or multiple fields" in 2G/3G/4G/5G. We use
//! the term to refer to a structured, fully parsed message in any telcom
//! standard.

use telcom_parser::{decode, lte_rrc};
use thiserror::Error;
use crate::gsmtap::{GsmtapType, LteRrcSubtype, GsmtapMessage};

#[derive(Error, Debug)]
pub enum InformationElementError {
    #[error("Failed decoding")]
    DecodingError(#[from] telcom_parser::ParsingError),
    #[error("Unsupported LTE RRC subtype {0:?}")]
    UnsupportedGsmtapType(GsmtapType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InformationElement {
    GSM,
    UMTS,
    LTE(LteInformationElement),
    FiveG,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LteInformationElement {
    DlCcch(lte_rrc::DL_CCCH_Message),
    DlDcch(lte_rrc::DL_DCCH_Message),
    UlCcch(lte_rrc::UL_CCCH_Message),
    UlDcch(lte_rrc::UL_DCCH_Message),
    BcchBch(lte_rrc::BCCH_BCH_Message),
    BcchDlSch(lte_rrc::BCCH_DL_SCH_Message),
    PCCH(lte_rrc::PCCH_Message),
    MCCH(lte_rrc::MCCH_Message),
    ScMcch(lte_rrc::SC_MCCH_Message_r13),
    BcchBchMbms(lte_rrc::BCCH_BCH_Message_MBMS),
    BcchDlSchBr(lte_rrc::BCCH_DL_SCH_Message_BR),
    BcchDlSchMbms(lte_rrc::BCCH_DL_SCH_Message_MBMS),
    SbcchSlBch(lte_rrc::SBCCH_SL_BCH_Message),
    SbcchSlBchV2x(lte_rrc::SBCCH_SL_BCH_Message_V2X_r14),

    // FIXME: unclear which message these "NB" types map to
    //DlCcchNb(),
    //DlDcchNb(),
    //UlCcchNb(),
    //UlDcchNb(),
    //BcchBchNb(),
    //BcchBchTddNb(),
    //BcchDlSchNb(),
    //PcchNb(),
    //ScMcchNb(),
}

impl TryFrom<&GsmtapMessage> for LteInformationElement {
    type Error = InformationElementError;

    fn try_from(gsmtap_msg: &GsmtapMessage) -> Result<Self, Self::Error> {
        if let GsmtapType::LteRrc(lte_rrc_subtype) = gsmtap_msg.header.gsmtap_type {
            use LteRrcSubtype as L;
            use LteInformationElement as R;
            return match lte_rrc_subtype {
                L::DlCcch => Ok(R::DlCcch(decode(&gsmtap_msg.payload)?)),
                L::DlDcch => Ok(R::DlDcch(decode(&gsmtap_msg.payload)?)),
                L::UlCcch => Ok(R::UlCcch(decode(&gsmtap_msg.payload)?)),
                L::UlDcch => Ok(R::UlDcch(decode(&gsmtap_msg.payload)?)),
                L::BcchBch => Ok(R::BcchBch(decode(&gsmtap_msg.payload)?)),
                L::BcchDlSch => Ok(R::BcchDlSch(decode(&gsmtap_msg.payload)?)),
                L::PCCH => Ok(R::PCCH(decode(&gsmtap_msg.payload)?)),
                L::MCCH => Ok(R::MCCH(decode(&gsmtap_msg.payload)?)),
                L::ScMcch => Ok(R::ScMcch(decode(&gsmtap_msg.payload)?)),
                L::BcchBchMbms => Ok(R::BcchBchMbms(decode(&gsmtap_msg.payload)?)),
                L::BcchDlSchBr => Ok(R::BcchDlSchBr(decode(&gsmtap_msg.payload)?)),
                L::BcchDlSchMbms => Ok(R::BcchDlSchMbms(decode(&gsmtap_msg.payload)?)),
                L::SbcchSlBch => Ok(R::SbcchSlBch(decode(&gsmtap_msg.payload)?)),
                L::SbcchSlBchV2x => Ok(R::SbcchSlBchV2x(decode(&gsmtap_msg.payload)?)),
                subtype => Err(InformationElementError::UnsupportedGsmtapType(gsmtap_msg.header.gsmtap_type)),
            };
        }
        Err(InformationElementError::UnsupportedGsmtapType(gsmtap_msg.header.gsmtap_type))
    }
}
