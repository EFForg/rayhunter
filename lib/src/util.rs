use nix::sys::utsname::uname;

/// Expose binary and system information.
pub struct RayhunterMetadata {
    pub name: String,
    pub version: String,
    pub os: String,
    pub arch: String,
    pub hardware: String,
}

impl RayhunterMetadata {
    pub fn new() -> Self {
        match uname() {
            Ok(utsname) => RayhunterMetadata {
                name: env!("CARGO_PKG_NAME").to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                arch: format!("{}", utsname.machine().to_string_lossy()),
                os: format!(
                    "{} {}",
                    utsname.sysname().to_string_lossy(),
                    utsname.release().to_string_lossy(),
                ),
                hardware: String::from("unknown"),
            },
            Err(_) => RayhunterMetadata {
                name: env!("CARGO_PKG_NAME").to_owned(),
                version: env!("CARGO_PKG_VERSION").to_owned(),
                arch: std::env::consts::ARCH.to_string(),
                os: std::env::consts::OS.to_string(),
                hardware: String::from("unknown"),
            },
        }
    }
}
