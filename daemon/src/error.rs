use thiserror::Error;

use crate::qmdl_store::RecordingStoreError;

#[derive(Error, Debug)]
pub enum RayhunterError {
    #[error("Config file parsing error: {0}")]
    ConfigFileParsingError(#[from] toml::de::Error),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::io::Error),
    #[error("QmdlStore error: {0}")]
    QmdlStoreError(#[from] RecordingStoreError),
    #[error("No QMDL store found at path {0}, but can't create a new one due to debug mode")]
    NoStoreDebugMode(String),
    #[error("Error parsing file to determine battery level")]
    BatteryLevelParseError,
    #[error("Error parsing file to determine whether device is plugged in")]
    BatteryPluggedInStatusParseError,
    #[error("The requested functionality is not supported for this device")]
    FunctionNotSupportedForDeviceError,
}
