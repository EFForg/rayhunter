use log::info;
use rayhunter::analysis::{
    analyzer::{Analyzer, EventType, Harness},
    wifi_oui_analyzer::WifiOUIAnalyzer,
};
use serde::Serialize;
use std::cmp;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::{
    fs::{self, File, try_exists},
    io::{self, AsyncWrite, AsyncWriteExt, BufWriter},
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
        let _ = self.writer.flush().await;
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

pub struct WifiAnalysisWriter {
    writer: BufWriter<File>,
    harness: Harness,
}

impl WifiAnalysisWriter {
    pub async fn new(file: File, wifi_ouis: &[String]) -> Result<Self, std::io::Error> {
        let mut harness = Harness::new();
        let wifi_analyzer = WifiOUIAnalyzer::new(wifi_ouis);
        harness.add_analyzer(WifiOUIAnalyzer::metadata(), Box::new(wifi_analyzer));

        let mut result = Self {
            writer: BufWriter::new(file),
            harness,
        };
        let metadata = result.harness.get_metadata();
        result.write(&metadata).await?;
        Ok(result)
    }

    pub async fn analyze_networks(
        &mut self,
        entry: &WifiScan,
    ) -> Result<EventType, std::io::Error> {
        let mut max_type = EventType::Informational;

        for network in entry.networks.iter() {
            let analysis_row = self
                .harness
                .analyze_wifi_network(&network.bssid, entry.start_ts);
            if !analysis_row.is_empty() {
                self.write(&analysis_row).await?;
                max_type = cmp::max(max_type, analysis_row.get_max_event_type());
            }
        }
        Ok(max_type)
    }

    async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut value_str = serde_json::to_string(value).unwrap();
        value_str.push('\n');
        self.writer.write_all(value_str.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    // Flushes any pending I/O to disk before dropping the writer
    pub async fn close(mut self) -> Result<(), WifiStoreError> {
        self.writer.flush().await.map_err(WifiStoreError::IOError)?;
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

    pub async fn write_scan_file(&self, scan: &WifiScan) -> Result<(), WifiStoreError> {
        let wifi_filepath =
            FileKind::Wifi.get_filepath(&format!("{}", scan.start_ts), &self.path, false);
        let wifi_file = File::create(&wifi_filepath)
            .await
            .map_err(WifiStoreError::CreateFileError)?;
        let mut wifi_writer = WifiWriter::new(wifi_file);
        wifi_writer.write(scan).await?;
        wifi_writer.close().await
    }

    pub async fn write_analysis_file(
        &self,
        scan: &WifiScan,
        wifi_ouis: &[String],
    ) -> Result<EventType, WifiStoreError> {
        let analysis_filepath =
            FileKind::Analysis.get_filepath(&format!("{}", scan.start_ts), &self.path, false);
        let analysis_file = File::create(&analysis_filepath)
            .await
            .map_err(WifiStoreError::CreateFileError)?;
        let mut analysis_writer = WifiAnalysisWriter::new(analysis_file, wifi_ouis)
            .await
            .map_err(WifiStoreError::IOError)?;
        let event_type = analysis_writer
            .analyze_networks(scan)
            .await
            .map_err(WifiStoreError::IOError)?;
        analysis_writer.close().await?;
        Ok(event_type)
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
