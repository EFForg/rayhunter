use log::warn;
use tokio::process::Command;

pub async fn block_ota_daemons() {
    let stub = "#!/bin/sh\nwhile true; do sleep 3600; done\n";
    if let Err(e) = tokio::fs::write("/tmp/daemon-stub", stub).await {
        warn!("failed to write daemon stub: {e}");
        return;
    }
    let _ = Command::new("chmod")
        .args(["755", "/tmp/daemon-stub"])
        .output()
        .await;

    for daemon in &["dmclient", "upgrade"] {
        let path = format!("/usr/bin/{daemon}");
        let _ = Command::new("mount")
            .args(["--bind", "/tmp/daemon-stub", &path])
            .output()
            .await;
        let _ = Command::new("pkill").args(["-9", daemon]).output().await;
    }
}
