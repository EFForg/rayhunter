use std::time::Duration;

use anyhow::{Context, Result, bail};

use crate::types::*;

pub struct RayhunterClient {
    client: reqwest::Client,
    base_url: String,
}

impl RayhunterClient {
    pub fn new(host: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");
        Self {
            client,
            base_url: format!("http://{host}"),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    pub async fn get_config(&self) -> Result<Config> {
        let resp = self
            .client
            .get(self.url("/api/config"))
            .send()
            .await
            .context("GET /api/config")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/config returned {status}");
        }
        resp.json().await.context("parsing config JSON")
    }

    pub async fn get_config_raw(&self) -> Result<String> {
        let resp = self
            .client
            .get(self.url("/api/config"))
            .send()
            .await
            .context("GET /api/config (raw)")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/config returned {status}");
        }
        resp.text().await.context("reading config body")
    }

    pub async fn set_config(&self, config: &Config) -> Result<()> {
        let resp = self
            .client
            .post(self.url("/api/config"))
            .json(config)
            .send()
            .await
            .context("POST /api/config")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/config returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let resp = self
            .client
            .get(self.url("/api/system-stats"))
            .send()
            .await
            .context("GET /api/system-stats")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/system-stats returned {status}");
        }
        resp.json().await.context("parsing system-stats JSON")
    }

    pub async fn get_time(&self) -> Result<TimeResponse> {
        let resp = self
            .client
            .get(self.url("/api/time"))
            .send()
            .await
            .context("GET /api/time")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/time returned {status}");
        }
        resp.json().await.context("parsing time JSON")
    }

    pub async fn set_time_offset(&self, offset_seconds: i64) -> Result<()> {
        let resp = self
            .client
            .post(self.url("/api/time-offset"))
            .json(&serde_json::json!({ "offset_seconds": offset_seconds }))
            .send()
            .await
            .context("POST /api/time-offset")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/time-offset returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn get_log(&self) -> Result<String> {
        let resp = self
            .client
            .get(self.url("/api/log"))
            .send()
            .await
            .context("GET /api/log")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/log returned {status}");
        }
        resp.text().await.context("reading log body")
    }

    pub async fn start_recording(&self) -> Result<()> {
        let resp = self
            .client
            .post(self.url("/api/start-recording"))
            .send()
            .await
            .context("POST /api/start-recording")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/start-recording returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn stop_recording(&self) -> Result<()> {
        let resp = self
            .client
            .post(self.url("/api/stop-recording"))
            .send()
            .await
            .context("POST /api/stop-recording")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/stop-recording returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn get_qmdl_manifest(&self) -> Result<ManifestStats> {
        let resp = self
            .client
            .get(self.url("/api/qmdl-manifest"))
            .send()
            .await
            .context("GET /api/qmdl-manifest")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/qmdl-manifest returned {status}");
        }
        resp.json().await.context("parsing manifest JSON")
    }

    pub async fn delete_recording(&self, name: &str) -> Result<()> {
        let resp = self
            .client
            .post(self.url(&format!("/api/delete-recording/{name}")))
            .send()
            .await
            .context("POST /api/delete-recording")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/delete-recording/{name} returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn delete_all_recordings(&self) -> Result<()> {
        let resp = self
            .client
            .post(self.url("/api/delete-all-recordings"))
            .send()
            .await
            .context("POST /api/delete-all-recordings")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/delete-all-recordings returned {status}: {body}");
        }
        Ok(())
    }

    pub async fn get_qmdl(&self, name: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get(self.url(&format!("/api/qmdl/{name}")))
            .send()
            .await
            .context("GET /api/qmdl")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET /api/qmdl/{name} returned {status}: {body}");
        }
        Ok(resp.bytes().await.context("reading QMDL bytes")?.to_vec())
    }

    pub async fn get_pcap(&self, name: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get(self.url(&format!("/api/pcap/{name}")))
            .send()
            .await
            .context("GET /api/pcap")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET /api/pcap/{name} returned {status}: {body}");
        }
        Ok(resp.bytes().await.context("reading PCAP bytes")?.to_vec())
    }

    pub async fn get_zip(&self, name: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get(self.url(&format!("/api/zip/{name}")))
            .send()
            .await
            .context("GET /api/zip")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET /api/zip/{name} returned {status}: {body}");
        }
        Ok(resp.bytes().await.context("reading ZIP bytes")?.to_vec())
    }

    pub async fn get_analysis(&self) -> Result<AnalysisStatus> {
        let resp = self
            .client
            .get(self.url("/api/analysis"))
            .send()
            .await
            .context("GET /api/analysis")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/analysis returned {status}");
        }
        resp.json().await.context("parsing analysis JSON")
    }

    pub async fn start_analysis(&self, name: &str) -> Result<AnalysisStatus> {
        let resp = self
            .client
            .post(self.url(&format!("/api/analysis/{name}")))
            .send()
            .await
            .context("POST /api/analysis")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/analysis/{name} returned {status}: {body}");
        }
        resp.json().await.context("parsing analysis JSON")
    }

    pub async fn get_analysis_report(&self, name: &str) -> Result<String> {
        let resp = self
            .client
            .get(self.url(&format!("/api/analysis-report/{name}")))
            .send()
            .await
            .context("GET /api/analysis-report")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET /api/analysis-report/{name} returned {status}: {body}");
        }
        resp.text().await.context("reading analysis report")
    }

    pub async fn get_wifi_status(&self) -> Result<WifiStatus> {
        let resp = self
            .client
            .get(self.url("/api/wifi-status"))
            .send()
            .await
            .context("GET /api/wifi-status")?;
        let status = resp.status();
        if !status.is_success() {
            bail!("GET /api/wifi-status returned {status}");
        }
        resp.json().await.context("parsing wifi-status JSON")
    }

    pub async fn scan_wifi(&self) -> Result<Vec<WifiNetwork>> {
        let resp = self
            .client
            .post(self.url("/api/wifi-scan"))
            .send()
            .await
            .context("POST /api/wifi-scan")?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST /api/wifi-scan returned {status}: {body}");
        }
        resp.json().await.context("parsing wifi-scan JSON")
    }

    pub async fn scan_wifi_raw(&self) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/wifi-scan"))
            .send()
            .await
            .context("POST /api/wifi-scan")
    }

    /// Create a recording with data and return its manifest entry name.
    /// Polls until qmdl_size_bytes > 0 rather than sleeping a fixed duration.
    /// Leaves the device in a stopped state.
    pub async fn create_recording(&self) -> Result<String> {
        let _ = self.stop_recording().await;
        tokio::time::sleep(Duration::from_millis(500)).await;

        self.start_recording().await?;

        let start = tokio::time::Instant::now();
        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;
            let manifest = self.get_qmdl_manifest().await?;
            if let Some(entry) = &manifest.current_entry
                && entry.qmdl_size_bytes > 0
            {
                break;
            }
            if start.elapsed() > Duration::from_secs(15) {
                bail!("recording did not capture any data within 15s");
            }
        }

        self.stop_recording().await?;
        tokio::time::sleep(Duration::from_millis(500)).await;

        let manifest = self.get_qmdl_manifest().await?;
        manifest
            .entries
            .last()
            .map(|e| e.name.clone())
            .ok_or_else(|| anyhow::anyhow!("no entries in manifest after recording"))
    }

    /// Create a recording, run a closure with its name, then delete it
    /// regardless of whether the closure succeeded or failed.
    pub async fn with_recording<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce(String) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let name = self.create_recording().await?;
        let result = f(name.clone()).await;
        let _ = self.delete_recording(&name).await;
        result
    }

    pub async fn get_qmdl_raw(&self, name: &str) -> Result<reqwest::Response> {
        self.client
            .get(self.url(&format!("/api/qmdl/{name}")))
            .send()
            .await
            .context("GET /api/qmdl")
    }

    pub async fn get_pcap_raw(&self, name: &str) -> Result<reqwest::Response> {
        self.client
            .get(self.url(&format!("/api/pcap/{name}")))
            .send()
            .await
            .context("GET /api/pcap")
    }

    pub async fn get_zip_raw(&self, name: &str) -> Result<reqwest::Response> {
        self.client
            .get(self.url(&format!("/api/zip/{name}")))
            .send()
            .await
            .context("GET /api/zip")
    }

    pub async fn get_analysis_report_raw(&self, name: &str) -> Result<reqwest::Response> {
        self.client
            .get(self.url(&format!("/api/analysis-report/{name}")))
            .send()
            .await
            .context("GET /api/analysis-report")
    }

    pub async fn start_recording_raw(&self) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/start-recording"))
            .send()
            .await
            .context("POST /api/start-recording")
    }

    pub async fn delete_recording_raw(&self, name: &str) -> Result<reqwest::Response> {
        self.client
            .post(self.url(&format!("/api/delete-recording/{name}")))
            .send()
            .await
            .context("POST /api/delete-recording")
    }

    pub async fn stop_recording_raw(&self) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/stop-recording"))
            .send()
            .await
            .context("POST /api/stop-recording")
    }

    pub async fn delete_all_recordings_raw(&self) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/delete-all-recordings"))
            .send()
            .await
            .context("POST /api/delete-all-recordings")
    }

    pub async fn post_time_offset_raw(&self, body: &str) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/time-offset"))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .context("POST /api/time-offset (raw)")
    }

    pub async fn post_config_raw(&self, body: &str) -> Result<reqwest::Response> {
        self.client
            .post(self.url("/api/config"))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await
            .context("POST /api/config (raw)")
    }

    pub async fn wait_for_ready(&self, timeout: Duration) -> Result<()> {
        let start = tokio::time::Instant::now();
        loop {
            if start.elapsed() > timeout {
                bail!("daemon did not become ready within {}s", timeout.as_secs());
            }
            match self.get_config().await {
                Ok(_) => return Ok(()),
                Err(_) => tokio::time::sleep(Duration::from_millis(500)).await,
            }
        }
    }
}
