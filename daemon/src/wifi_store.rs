use log::info;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::{
    fs::{self, OpenOptions, try_exists},
    io::{self, AsyncWrite, AsyncWriteExt},
};

use crate::{
    diag::{DiskSpaceCheck, check_disk_space},
    qmdl_store::FileKind,
    wifi_scan::WifiScan,
};

pub struct WifiWriter<T>
where
    T: AsyncWrite + Unpin,
{
    writer: T,
}

impl<T> WifiWriter<T>
where
    T: AsyncWrite + Unpin,
{
    pub fn new(writer: T) -> Self {
        Self { writer }
    }

    pub async fn write(&mut self, entry: &WifiScan) -> Result<(), WifiStoreError> {
        let json = serde_json::to_string(&entry).map_err(WifiStoreError::JsonWriteError)?;
        self.writer
            .write_all(json.as_bytes())
            .await
            .map_err(WifiStoreError::IOError)?;
        self.writer.flush().await.map_err(WifiStoreError::IOError)?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), WifiStoreError> {
        self.writer
            .shutdown()
            .await
            .map_err(WifiStoreError::IOError)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum WifiStoreError {
    #[error("Couldn't open directory at path: {0}")]
    OpenDirError(tokio::io::Error),
    #[error("Couldn't create file: {0}")]
    CreateFileError(tokio::io::Error),
    #[error("Insufficient disk space: {0}MB available, {1}MB required")]
    InsufficientDiskSpace(u64, u64),
    #[error("Error writing to wifi file: {0}")]
    JsonWriteError(serde_json::Error),
    #[error("I/O Error writing to wifi file: {0}")]
    IOError(io::Error),
}

pub struct WifiStore {
    pub path: PathBuf,
}

impl WifiStore {
    pub async fn exists<P>(path: P) -> Result<bool, WifiStoreError>
    where
        P: AsRef<Path>,
    {
        let dir_exists = try_exists(path)
            .await
            .map_err(WifiStoreError::OpenDirError)?;
        Ok(dir_exists)
    }

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

    pub async fn new<P>(path: P) -> Result<WifiStore, WifiStoreError>
    where
        P: AsRef<Path>,
    {
        Ok(Self {
            path: path.as_ref().to_path_buf(),
        })
    }

    pub async fn write_scan_file(
        &self,
        scan: &WifiScan,
        entry_name: &str,
    ) -> Result<(), WifiStoreError> {
        let wifi_filepath = FileKind::Wifi.get_filepath(entry_name, &self.path, false);
        info!("Writing scan {:?} to wifi file {:?}", scan, wifi_filepath);
        let wifi_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&wifi_filepath)
            .await
            .map_err(WifiStoreError::CreateFileError)?;
        let mut wifi_writer = WifiWriter::new(wifi_file);
        wifi_writer.write(scan).await?;
        wifi_writer.close().await
    }

    pub async fn check_disk_space(
        &self,
        min_space_to_start_mb: u64,
        min_space_to_continue_mb: u64,
    ) -> Result<(), WifiStoreError> {
        match check_disk_space(&self.path, min_space_to_start_mb, min_space_to_continue_mb) {
            DiskSpaceCheck::Critical(mb) | DiskSpaceCheck::Warning(mb) => Err(
                WifiStoreError::InsufficientDiskSpace(mb, min_space_to_start_mb),
            ),
            DiskSpaceCheck::Ok(mb) => {
                info!("Starting wifi recording with {}MB disk space available", mb);
                Ok(())
            }
            DiskSpaceCheck::Failed => Ok(()),
        }
    }
}
