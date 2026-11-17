use chrono::{DateTime, FixedOffset};
use log::debug;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::DeviceMetadata;
use crate::diag::{DiagParsingError, Message, MessagesContainer};
use crate::gsmtap::{GsmtapHeader, GsmtapMessage, GsmtapType, parser as gsmtap_parser};
use crate::util::RuntimeMetadata;

use super::{
    connection_redirect_downgrade::ConnectionRedirect2GDowngradeAnalyzer,
    diagnostic::DiagnosticAnalyzer, imsi_requested::ImsiRequestedAnalyzer,
    incomplete_sib::IncompleteSibAnalyzer, information_element::InformationElement,
    markov_entropy::MarkovRrcEntropyAnalyzer, nas_null_cipher::NasNullCipherAnalyzer,
    no_nas_messages::NoNasMessagesAnalyzer, null_cipher::NullCipherAnalyzer,
    priority_2g_downgrade::LteSib6And7DowngradeAnalyzer, test_analyzer::TestAnalyzer,
};

fn boxed(analyzer: impl Analyzer + Send + 'static) -> Box<dyn Analyzer + Send> {
    Box::new(analyzer)
}

fn all_analyzers(
    device_metadata: &DeviceMetadata,
) -> impl Iterator<Item = (AnalyzerMetadata, Box<dyn Analyzer + Send>)> {
    [
        (
            ImsiRequestedAnalyzer::metadata(),
            boxed(ImsiRequestedAnalyzer::new(
                device_metadata.home_plmn.clone(),
            )),
        ),
        (
            ConnectionRedirect2GDowngradeAnalyzer::metadata(),
            boxed(ConnectionRedirect2GDowngradeAnalyzer {}),
        ),
        (
            LteSib6And7DowngradeAnalyzer::metadata(),
            boxed(LteSib6And7DowngradeAnalyzer::new()),
        ),
        (NullCipherAnalyzer::metadata(), boxed(NullCipherAnalyzer {})),
        (
            NasNullCipherAnalyzer::metadata(),
            boxed(NasNullCipherAnalyzer {}),
        ),
        (
            IncompleteSibAnalyzer::metadata(),
            boxed(IncompleteSibAnalyzer {}),
        ),
        (TestAnalyzer::metadata(), boxed(TestAnalyzer {})),
        (
            NoNasMessagesAnalyzer::metadata(),
            boxed(NoNasMessagesAnalyzer::new()),
        ),
        (DiagnosticAnalyzer::metadata(), boxed(DiagnosticAnalyzer {})),
        (
            MarkovRrcEntropyAnalyzer::metadata(),
            boxed(MarkovRrcEntropyAnalyzer::new()),
        ),
    ]
    .into_iter()
}
