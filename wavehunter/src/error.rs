use thiserror::Error;
use orca::diag_device::DiagDeviceError;

#[derive(Error, Debug)]
pub enum WavehunterError {
    #[error("Missing config file: {0}")]
    MissingConfigFile(String),
    #[error("Config file parsing error: {0}")]
    ConfigFileParsingError(#[from] toml::de::Error),
    #[error("Diag intialization error: {0}")]
    DiagInitError(DiagDeviceError),
    #[error("Diag read error: {0}")]
    DiagReadError(DiagDeviceError),
    #[error("Tokio error: {0}")]
    TokioError(#[from] tokio::io::Error),
}
