//! Shared constructors for the daemon's outbound HTTP client.
//!
//! All `reqwest` clients in the daemon are created here so they share a
//! consistent User-Agent and a default timeout (see #1057).

use std::time::Duration;

/// Default request timeout for the shared HTTP client. Individual requests can
/// still override this with `RequestBuilder::timeout`.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// The User-Agent sent with every outbound HTTP request from the daemon.
pub fn user_agent() -> String {
    format!("rayhunter/{}", env!("CARGO_PKG_VERSION"))
}

/// A [`reqwest::ClientBuilder`] preconfigured with the shared User-Agent and
/// default timeout. Use this when a caller needs to customize the client
/// further (e.g. a longer upload timeout); otherwise prefer [`client`].
pub fn builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .user_agent(user_agent())
        .timeout(DEFAULT_TIMEOUT)
}

/// Builds a [`reqwest::Client`] with the shared User-Agent and default timeout.
pub fn client() -> reqwest::Result<reqwest::Client> {
    builder().build()
}
