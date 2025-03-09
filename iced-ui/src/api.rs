use anyhow::Result;
use chrono::{DateTime, Local};
use iced::Command;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskStats {
    pub partition: String,
    pub total_size: String,
    pub used_size: String,
    pub available_size: String,
    pub used_percent: String,
    pub mounted_on: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryStats {
    pub total: String,
    pub used: String,
    pub free: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStats {
    pub disk_stats: DiskStats,
    pub memory_stats: MemoryStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestEntry {
    pub name: String,
    pub start_time: DateTime<Local>,
    pub last_message_time: Option<DateTime<Local>>,
    pub qmdl_size_bytes: usize,
    pub analysis_size_bytes: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ManifestStats {
    pub entries: Vec<ManifestEntry>,
    pub current_entry: Option<ManifestEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnalysisStatus {
    pub queued: Vec<String>,
    pub running: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub fn get_system_stats(&self) -> Command<Result<SystemStats, String>> {
        let client = self.client.clone();
        let url = format!("{}/api/system-stats", self.base_url);

        Command::perform(
            async move {
                client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<SystemStats>()
                    .await
                    .map_err(|e| e.to_string())
            },
            |result| result,
        )
    }

    pub fn get_qmdl_manifest(&self) -> Command<Result<ManifestStats, String>> {
        let client = self.client.clone();
        let url = format!("{}/api/qmdl-manifest", self.base_url);

        Command::perform(
            async move {
                client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<ManifestStats>()
                    .await
                    .map_err(|e| e.to_string())
            },
            |result| result,
        )
    }

    pub fn get_analysis_status(&self) -> Command<Result<AnalysisStatus, String>> {
        let client = self.client.clone();
        let url = format!("{}/api/analysis", self.base_url);

        Command::perform(
            async move {
                client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .json::<AnalysisStatus>()
                    .await
                    .map_err(|e| e.to_string())
            },
            |result| result,
        )
    }

    pub fn start_recording(&self) -> Command<Result<(), String>> {
        let client = self.client.clone();
        let url = format!("{}/api/start-recording", self.base_url);

        Command::perform(
            async move {
                client
                    .post(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .error_for_status()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            |result| result,
        )
    }

    pub fn stop_recording(&self) -> Command<Result<(), String>> {
        let client = self.client.clone();
        let url = format!("{}/api/stop-recording", self.base_url);

        Command::perform(
            async move {
                client
                    .post(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .error_for_status()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            |result| result,
        )
    }

    pub fn start_analysis(&self, qmdl_name: &str) -> Command<Result<(), String>> {
        let client = self.client.clone();
        let url = format!("{}/api/analysis/{}", self.base_url, qmdl_name);

        Command::perform(
            async move {
                client
                    .post(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .error_for_status()
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            |result| result,
        )
    }

    pub fn get_analysis_report(&self, qmdl_name: &str) -> Command<Result<String, String>> {
        let client = self.client.clone();
        let url = format!("{}/api/analysis-report/{}", self.base_url, qmdl_name);

        Command::perform(
            async move {
                client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
                    .text()
                    .await
                    .map_err(|e| e.to_string())
            },
            |result| result,
        )
    }

    pub fn get_pcap_url(&self, qmdl_name: &str) -> String {
        format!("{}/api/pcap/{}", self.base_url, qmdl_name)
    }

    pub fn get_qmdl_url(&self, qmdl_name: &str) -> String {
        format!("{}/api/qmdl/{}.qmdl", self.base_url, qmdl_name)
    }
}