use chrono::{DateTime, FixedOffset};

use pycrate_rs::nas::{
    NASMessage::EMMMessage,
    emm::EMMMessage::{EMMDLGenericNASTransport, EMMULGenericNASTransport},
    generated::emm::{
        emmdl_generic_nas_transport::GenericContTypeGenericContType::LTEPositioningProtocolLPPMessageContainer as EMMDL_LTEPositioningProtocolLPPMessageContainer,
        emmul_generic_nas_transport::GenericContTypeGenericContType::LTEPositioningProtocolLPPMessageContainer as EMMUL_LTEPositioningProtocolLPPMessageContainer,
    },
    layer3::{Layer3Buffer, Type4TLV, Type6LVE},
};
use telcom_parser::{
    ParsingError,
    lte_lpp::{LPP_Message, LPP_MessageBody},
};

use crate::analysis::{
    analyzer::{Analyzer, AnalyzerMetadata, Event},
    information_element::{
        InformationElement::{self, LTE},
        LteInformationElement::NAS,
    },
    nas_lpp::{Stage, State, TransactionKey},
};

/// Looks across packages of supported types for the presence of [LPP_Message] buffers.
/// Tracks messages according to their LPP transaction within an LPP session.
/// See State for the state machine rules.
///
/// Events (Informational):
/// - summary events for ended LPP transactions (including timeouts and errors)
/// - rate-limited progress events for ongoing LPP transactions
/// - other events for decoding failures
///
/// Supported Types
/// - [EMMDL_LTEPositioningProtocolLPPMessageContainer]
/// - [EMMUL_LTEPositioningProtocolLPPMessageContainer]
///
/// limitations:
/// - No support for packets of type LocationServicesMessageContainer, which contain
///   more LPP messages (potentially in batch). Transactions over this communication
///   may be missed, or produce partially captured transactions.
///   Requires ASN.1 and bindings from TS 24.080
///
/// ```text
/// "LPP::Transaction[deadbeef/s000]::Progress[RequestLocationInformation -> ProvideLocationInformation x2]"
/// "LPP::Transaction[deadbeef/s000]::Summary[RequestCapabilities -> ProvideCapabilities -> Abort(TARGET_DEVICE_ABORT)]"
/// "LPP::Transaction[deadbeef/----]::Error(...)"
/// ```
#[derive(Default)]
pub struct NasLppAnalyzer {
    state: State,
}

impl Analyzer for NasLppAnalyzer {
    fn metadata() -> AnalyzerMetadata {
        AnalyzerMetadata {
            key: "nas_lpp".into(),
            default_enabled: true,
            name: "NAS LTE Positioning Analyzer".into(),
            description:
                "An information-only analyzer that captures the lifecycle of LPP transactions."
                    .into(),
            version: 0,
        }
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
        timestamp: DateTime<FixedOffset>,
    ) -> Option<Event> {
        let message = match ie {
            LTE(inner) => match &**inner {
                NAS(EMMMessage(EMMDLGenericNASTransport(inner)))
                    if inner.generic_cont_type.inner
                        == EMMDL_LTEPositioningProtocolLPPMessageContainer =>
                {
                    Some(decode(&inner.generic_container, &inner.add_info))
                }

                NAS(EMMMessage(EMMULGenericNASTransport(inner)))
                    if inner.generic_cont_type.inner
                        == EMMUL_LTEPositioningProtocolLPPMessageContainer =>
                {
                    Some(decode(&inner.generic_container, &inner.add_info))
                }

                _ => None,
            },
            _ => None,
        };

        match message {
            Some((Ok(message), route_id)) => {
                if let Some(LPP_MessageBody::C1(body)) = message.lpp_message_body {
                    let transaction_id = message.transaction_id;
                    let key = TransactionKey::new(route_id, transaction_id);
                    let stage = Stage::from(body);
                    let end = message.end_transaction.0;
                    self.state.accept(timestamp, key, stage, end);
                }
            }
            Some((Err(e), route_id)) => self.state.accept_err(route_id, e),
            _ => (),
        }

        self.state.next(timestamp)
    }

    fn report_skipped_packet(&mut self, timestamp: DateTime<FixedOffset>) -> Option<Event> {
        self.state.next(timestamp)
    }
}

/// add_info (Additional Information): TS 24.171: 5.3.2.1.1
/// Routing Identifier (or "Correlation identifier", TS 29.171: 7.4.28)
/// This IE indicates a specific location session.
/// It is used in order to correlate request/response and an ongoing location session.
fn decode(
    generic_container: &Type6LVE<Layer3Buffer>,
    add_info: &Type4TLV<Layer3Buffer>,
) -> (Result<LPP_Message, ParsingError>, Option<Vec<u8>>) {
    let msg = telcom_parser::decode(&generic_container.inner.buf);
    let info = add_info.inner.as_ref().map(|b| b.buf.clone());
    (msg, info)
}
