use std::future::Future;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use libtest_mimic::Failed;

use crate::capabilities::Capabilities;
use crate::client::RayhunterClient;
use crate::shell::AdbShell;

pub struct TestContext {
    pub client: RayhunterClient,
    pub capabilities: Capabilities,
    pub shell: Option<AdbShell>,
}

static CONTEXT: OnceLock<Arc<TestContext>> = OnceLock::new();

pub fn set_context(ctx: Arc<TestContext>) {
    CONTEXT
        .set(ctx)
        .unwrap_or_else(|_| panic!("context already initialized"));
}

pub fn ctx() -> &'static Arc<TestContext> {
    CONTEXT.get().expect("context not initialized")
}

pub fn run(future: impl Future<Output = anyhow::Result<()>>) -> Result<(), Failed> {
    run_with_timeout(Duration::from_secs(30), future)
}

pub fn run_slow(future: impl Future<Output = anyhow::Result<()>>) -> Result<(), Failed> {
    run_with_timeout(Duration::from_secs(120), future)
}

fn run_with_timeout(
    timeout: Duration,
    future: impl Future<Output = anyhow::Result<()>>,
) -> Result<(), Failed> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    match rt.block_on(tokio::time::timeout(timeout, future)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("{e:#}").into()),
        Err(_) => Err(format!("timed out ({}s)", timeout.as_secs()).into()),
    }
}
