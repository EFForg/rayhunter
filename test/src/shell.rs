use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Result, bail, ensure};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub const DEVICE_CONFIG_PATH: &str = "/data/rayhunter/config.toml";
pub const DEVICE_WPA_CONF_PATH: &str = "/data/rayhunter/wpa_sta.conf";
pub const DEVICE_LOG_PATH: &str = "/data/rayhunter/rayhunter.log";

pub trait ShellConnection: Send + Sync {
    fn run_command(
        &self,
        command: &str,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    fn read_file(
        &self,
        remote_path: &str,
    ) -> impl std::future::Future<Output = Result<Option<String>>> + Send;

    fn write_file(
        &self,
        remote_path: &str,
        content: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    fn remove_file(
        &self,
        remote_path: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub struct AdbShell;

impl ShellConnection for AdbShell {
    async fn run_command(&self, command: &str) -> Result<String> {
        let output = tokio::process::Command::new("adb")
            .args(["shell", command])
            .output()
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("adb shell command failed: {stderr}");
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    async fn read_file(&self, remote_path: &str) -> Result<Option<String>> {
        let output = tokio::process::Command::new("adb")
            .args(["shell", &format!("cat {remote_path} 2>/dev/null")])
            .output()
            .await?;
        if !output.status.success() {
            return Ok(None);
        }
        let content = String::from_utf8_lossy(&output.stdout).into_owned();
        if content.is_empty() {
            return Ok(None);
        }
        Ok(Some(content))
    }

    async fn write_file(&self, remote_path: &str, content: &str) -> Result<()> {
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let tmp =
            std::env::temp_dir().join(format!("rayhunter-test-{}-{}", std::process::id(), seq));
        std::fs::write(&tmp, content)?;

        let output = tokio::process::Command::new("adb")
            .args(["push", tmp.to_str().unwrap(), remote_path])
            .output()
            .await?;
        let _ = std::fs::remove_file(&tmp);

        ensure!(
            output.status.success(),
            "adb push failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    async fn remove_file(&self, remote_path: &str) -> Result<()> {
        let _ = self.run_command(&format!("rm -f {remote_path}")).await;
        Ok(())
    }
}
