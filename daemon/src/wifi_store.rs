use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::{self, File};

use crate::qmdl_store::FileKind;

#[derive(Debug, Error)]
pub enum WifiStoreError {
    #[error("Couldn't open directory at path: {0}")]
    OpenDirError(tokio::io::Error),
    #[error("Couldn't create file: {0}")]
    CreateFileError(tokio::io::Error),
}

pub struct WifiStore {
    pub path: PathBuf,
}

impl WifiStore {
    pub async fn create<P>(path: P) -> Result<Self, WifiStoreError>
    where
        P: AsRef<Path>,
    {
        fs::create_dir_all(&path)
            .await
            .map_err(WifiStoreError::OpenDirError)?;

        let store = Self {
            path: path.as_ref().to_owned(),
        };

        Ok(store)
    }

    pub async fn new_entry(
        &mut self,
    ) -> Result<(File, File), WifiStoreError> {
        // if we've already got an entry open, close it
        let now = rayhunter::clock::get_adjusted_now();
        let wifi_filepath = FileKind::Wifi.get_filepath(&format!("{}", now.timestamp()), &self.path, false);
        let wifi_file = File::create(&wifi_filepath)
            .await
            .map_err(WifiStoreError::CreateFileError)?;
        let analysis_filepath = FileKind::Analysis.get_filepath(&format!("{}", now.timestamp()), &self.path, false);
        let analysis_file = File::create(&analysis_filepath)
            .await
            .map_err(WifiStoreError::CreateFileError)?;
        Ok((wifi_file, analysis_file))
    }
}
