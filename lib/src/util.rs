use nix::sys::utsname::uname;
use serde::Serialize;

/// Expose binary and system information.
#[derive(Serialize, Debug)]
pub struct RuntimeMetadata {
    /// The cargo package version from this library's cargo.toml, e.g., "1.2.3".
    pub rayhunter_version: String,
    /// The operating system `sysname` and optionally `release`. e.g., "Linux 3.18.48" or "linux".
    pub system_os: String,
    /// The CPU architecture in use. e.g., "armv7l" or "arm".
    pub arch: String,
}

impl Default for RuntimeMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeMetadata {
    /// Return the binary and system information, attempting to retrieve
    /// attributes from `uname(2)` and falling back to values from
    /// `std::env::consts`.
    pub fn new() -> Self {
        match uname() {
            Ok(utsname) => RuntimeMetadata {
                rayhunter_version: env!("CARGO_PKG_VERSION").to_owned(),
                arch: format!("{}", utsname.machine().to_string_lossy()),
                system_os: format!(
                    "{} {}",
                    utsname.sysname().to_string_lossy(),
                    utsname.release().to_string_lossy(),
                ),
            },
            Err(_) => RuntimeMetadata {
                rayhunter_version: env!("CARGO_PKG_VERSION").to_owned(),
                arch: std::env::consts::ARCH.to_string(),
                system_os: std::env::consts::OS.to_string(),
            },
        }
    }
}
