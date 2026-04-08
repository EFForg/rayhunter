use std::sync::Once;

static INSTALL: Once = Once::new();

/// Install the default rustls `CryptoProvider` for the current process.
///
/// This is idempotent so that it's easier to use in tests, but also panics loudly if the
/// initialization fails.
pub fn install_default() {
    // Crypto providers fail if they get initialized multiple times, but we don't want to just
    // ignore all errors, hence the use of once.
    INSTALL.call_once(|| {
        #[cfg(feature = "rustcrypto-tls")]
        rustls_rustcrypto::provider()
            .install_default()
            .expect("failed to install rustcrypto crypto provider");

        #[cfg(feature = "pq-tls")]
        rustls_post_quantum::provider()
            .install_default()
            .expect("failed to install aws-lc-rs post-quantum crypto provider");
    });
}
