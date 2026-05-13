use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tempfile::TempDir;

const STARTUP_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

struct DaemonGuard {
    child: Option<Child>,
    stderr: Arc<Mutex<Vec<u8>>>,
}

impl DaemonGuard {
    fn stderr_dump(&self) -> String {
        let buf = self.stderr.lock().unwrap();
        String::from_utf8_lossy(&buf).into_owned()
    }

    fn shutdown(&mut self, timeout: Duration) -> std::io::Result<ExitStatus> {
        let mut child = self
            .child
            .take()
            .expect("daemon already shut down or never started");

        #[cfg(unix)]
        {
            let pid = child.id() as libc::pid_t;
            // SAFETY: child.id() returns the OS pid of a process we own.
            unsafe { libc::kill(pid, libc::SIGINT) };
        }
        #[cfg(not(unix))]
        {
            let _ = child.kill();
        }

        let start = Instant::now();
        loop {
            match child.try_wait()? {
                Some(status) => return Ok(status),
                None => {
                    if start.elapsed() >= timeout {
                        let _ = child.kill();
                        return child.wait();
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
}

impl Drop for DaemonGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn pick_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local_addr").port()
}

fn wait_for_port(port: u16, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    false
}

#[tokio::test(flavor = "current_thread")]
async fn daemon_serves_index_and_api() {
    let port = pick_free_port();

    let tmp = TempDir::new().unwrap();
    let qmdl_dir = tmp.path().join("qmdl");
    std::fs::create_dir(&qmdl_dir).unwrap();
    // The daemon refuses to create a store in debug_mode, so seed an empty
    // manifest. See init_qmdl_store in daemon/src/main.rs.
    std::fs::write(qmdl_dir.join("manifest.toml"), "entries = []\n").unwrap();

    let config_path = tmp.path().join("config.toml");
    std::fs::write(
        &config_path,
        format!(
            "qmdl_store_path = \"{}\"\nport = {}\ndebug_mode = true\n",
            qmdl_dir.display(),
            port,
        ),
    )
    .unwrap();

    let daemon_bin = env!("CARGO_BIN_EXE_rayhunter-daemon");
    let mut child = Command::new(daemon_bin)
        .arg(&config_path)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn daemon");

    let stderr_buf: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    if let Some(mut pipe) = child.stderr.take() {
        let sink = stderr_buf.clone();
        std::thread::spawn(move || {
            let mut chunk = [0u8; 4096];
            loop {
                match pipe.read(&mut chunk) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => sink.lock().unwrap().extend_from_slice(&chunk[..n]),
                }
            }
        });
    }

    let mut guard = DaemonGuard {
        child: Some(child),
        stderr: stderr_buf,
    };

    if !wait_for_port(port, STARTUP_TIMEOUT) {
        panic!(
            "daemon did not start listening on {port} within {STARTUP_TIMEOUT:?}\n--- daemon stderr ---\n{}",
            guard.stderr_dump(),
        );
    }

    // reqwest's rustls backend gets pulled in via feature unification with the
    // daemon's production deps. The test process needs its own crypto provider.
    rayhunter_daemon::crypto_provider::install_default();

    let client = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .gzip(true)
        .build()
        .unwrap();
    let base = format!("http://127.0.0.1:{port}");

    let resp = client
        .get(format!("{base}/index.html"))
        .send()
        .await
        .expect("GET /index.html failed");
    assert!(
        resp.status().is_success(),
        "GET /index.html returned {}",
        resp.status(),
    );
    let body = resp.text().await.expect("could not read index.html body");
    assert!(
        body.contains("Rayhunter"),
        "decompressed index.html body did not contain 'Rayhunter' marker (len={})",
        body.len(),
    );

    let resp = client
        .get(format!("{base}/api/qmdl-manifest"))
        .send()
        .await
        .expect("GET /api/qmdl-manifest failed");
    assert!(
        resp.status().is_success(),
        "GET /api/qmdl-manifest returned {}",
        resp.status(),
    );

    let status = guard
        .shutdown(SHUTDOWN_TIMEOUT)
        .expect("waiting for daemon exit failed");
    assert!(
        status.success(),
        "daemon did not exit cleanly after SIGINT: {status}\n--- daemon stderr ---\n{}",
        guard.stderr_dump(),
    );
}
