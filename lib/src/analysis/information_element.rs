//! The term "information element" is used by 3GPP to describe "structural
//! elements containing single or multiple fields" in 2G/3G/4G/5G. We use
//! the term to refer to a structured, fully parsed message in any telcom
//! standard.

use telcom_parser::{decode, lte_rrc};
use thiserror::Error;
use crate::gsmtap::{GsmtapMessage, GsmtapType, LteNasSubtype, LteRrcSubtype};

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

    // FIXME: actually parse NAS messages
    NAS(Vec<u8>),

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

impl TryFrom<&GsmtapMessage> for InformationElement {
    type Error = InformationElementError;

    fn try_from(gsmtap_msg: &GsmtapMessage) -> Result<Self, Self::Error> {
        match gsmtap_msg.header.gsmtap_type {
            GsmtapType::LteRrc(lte_rrc_subtype) => {
                use LteRrcSubtype as L;
                use LteInformationElement as R;
                let lte = match lte_rrc_subtype {
                    L::DlCcch => R::DlCcch(decode(&gsmtap_msg.payload)?),
                    L::DlDcch => R::DlDcch(decode(&gsmtap_msg.payload)?),
                    L::UlCcch => R::UlCcch(decode(&gsmtap_msg.payload)?),
                    L::UlDcch => R::UlDcch(decode(&gsmtap_msg.payload)?),
                    L::BcchBch => R::BcchBch(decode(&gsmtap_msg.payload)?),
                    L::BcchDlSch => R::BcchDlSch(decode(&gsmtap_msg.payload)?),
                    L::PCCH => R::PCCH(decode(&gsmtap_msg.payload)?),
                    L::MCCH => R::MCCH(decode(&gsmtap_msg.payload)?),
                    L::ScMcch => R::ScMcch(decode(&gsmtap_msg.payload)?),
                    L::BcchBchMbms => R::BcchBchMbms(decode(&gsmtap_msg.payload)?),
                    L::BcchDlSchBr => R::BcchDlSchBr(decode(&gsmtap_msg.payload)?),
                    L::BcchDlSchMbms => R::BcchDlSchMbms(decode(&gsmtap_msg.payload)?),
                    L::SbcchSlBch => R::SbcchSlBch(decode(&gsmtap_msg.payload)?),
                    L::SbcchSlBchV2x => R::SbcchSlBchV2x(decode(&gsmtap_msg.payload)?),
                    _ => return Err(InformationElementError::UnsupportedGsmtapType(gsmtap_msg.header.gsmtap_type)),
                };
                Ok(InformationElement::LTE(lte))
            },
            GsmtapType::LteNas(LteNasSubtype::Plain) => {
                Ok(InformationElement::LTE(LteInformationElement::NAS(gsmtap_msg.payload.clone())))
            },
            _ => Err(InformationElementError::UnsupportedGsmtapType(gsmtap_msg.header.gsmtap_type)),
        }
    }
}
