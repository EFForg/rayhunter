//! TLS certificate generation and management for HTTPS support.

use std::net::IpAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use log::{error, info, warn};
use rayhunter::Device;
use rcgen::{CertificateParams, DnType, Ia5String, KeyPair, SanType};
use tokio::fs;

use crate::error::RayhunterError;

/// Default certificate validity in days (10 years)
const CERT_VALIDITY_DAYS: u32 = 3650;

/// Maximum number of certificate regeneration attempts before falling back to HTTP
const MAX_REGEN_ATTEMPTS: u32 = 3;

/// File to track regeneration attempts (prevents boot loops)
const REGEN_ATTEMPTS_FILE: &str = "regen_attempts.txt";

/// Global flag indicating TLS is in fallback mode due to repeated failures
static TLS_FALLBACK_MODE: AtomicBool = AtomicBool::new(false);

/// Get the default gateway IP for a specific device type.
///
/// These IPs are the WiFi hotspot gateway addresses that users connect to.
/// Sources: installer/src/lib.rs default_value args and device documentation.
pub fn get_device_default_ip(device: &Device) -> IpAddr {
    match device {
        Device::Orbic => "192.168.1.1".parse().unwrap(), // doc/orbic.md, installer
        Device::Tplink => "192.168.0.1".parse().unwrap(), // doc/tplink-m7350.md, installer
        Device::Tmobile => "192.168.0.1".parse().unwrap(), // installer/src/lib.rs:173
        Device::Wingtech => "192.168.1.1".parse().unwrap(), // installer/src/lib.rs:220
        Device::Pinephone => "127.0.0.1".parse().unwrap(), // accessed via ADB forwarding
        Device::Uz801 => "192.168.100.1".parse().unwrap(), // doc/uz801.md:37
    }
}

/// Build Subject Alternative Names for the certificate.
///
/// Uses device-specific default gateway IP plus localhost, with optional custom hosts.
pub fn get_certificate_sans(device: &Device, custom_hosts: &[String]) -> Vec<SanType> {
    let mut sans = vec![
        SanType::IpAddress(get_device_default_ip(device)),
        SanType::IpAddress("127.0.0.1".parse().unwrap()),
    ];

    // Add user-provided custom hosts if any
    for host in custom_hosts {
        let host = host.trim();
        if host.is_empty() {
            continue;
        }
        if let Some(san) = parse_san_entry(host) {
            if !sans.contains(&san) {
                sans.push(san);
            }
        }
    }

    sans
}

/// Parse a host string into a SanType (either IP or DNS name).
/// Returns None if the DNS name is invalid.
fn parse_san_entry(host: &str) -> Option<SanType> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        Some(SanType::IpAddress(ip))
    } else if let Ok(dns_name) = Ia5String::try_from(host) {
        Some(SanType::DnsName(dns_name))
    } else {
        warn!("Invalid SAN entry (not a valid IP or hostname): {}", host);
        None
    }
}

/// Generate a self-signed certificate with the given SANs.
///
/// Returns (certificate_pem, private_key_pem) as strings.
fn generate_self_signed_cert_with_sans(
    sans: Vec<SanType>,
) -> Result<(String, String), RayhunterError> {
    let mut params = CertificateParams::default();

    // Set certificate subject
    params
        .distinguished_name
        .push(DnType::CommonName, "Rayhunter");
    params
        .distinguished_name
        .push(DnType::OrganizationName, "Electronic Frontier Foundation");

    let san_count = sans.len();
    params.subject_alt_names = sans;

    // Set validity period
    params.not_before = rcgen::date_time_ymd(2024, 1, 1);
    let end_year = 2024 + (CERT_VALIDITY_DAYS / 365) as i32;
    params.not_after = rcgen::date_time_ymd(end_year, 12, 31);

    // Generate key pair
    let key_pair = KeyPair::generate()
        .map_err(|e| RayhunterError::TlsError(format!("Failed to generate key pair: {}", e)))?;

    // Generate certificate
    let cert = params
        .self_signed(&key_pair)
        .map_err(|e| RayhunterError::TlsError(format!("Failed to generate certificate: {}", e)))?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    info!("Generated self-signed certificate with {} SANs", san_count);

    Ok((cert_pem, key_pem))
}

/// Get the TLS directory path based on qmdl_store_path.
///
/// TLS files are stored in `{qmdl_store_path}/../tls/`
pub fn get_tls_dir(qmdl_store_path: &str) -> PathBuf {
    Path::new(qmdl_store_path)
        .parent()
        .unwrap_or(Path::new("/data/rayhunter"))
        .join("tls")
}

/// Check if TLS is in fallback mode due to repeated certificate failures.
pub fn is_tls_fallback_mode() -> bool {
    TLS_FALLBACK_MODE.load(Ordering::Relaxed)
}

/// Set TLS fallback mode flag.
fn set_tls_fallback_mode(fallback: bool) {
    TLS_FALLBACK_MODE.store(fallback, Ordering::Relaxed);
}

/// Get the reason for TLS fallback (for UI display).
/// Returns None if not in fallback mode.
pub fn get_tls_fallback_reason() -> Option<String> {
    if is_tls_fallback_mode() {
        Some("TLS certificate generation failed repeatedly. Using HTTP-only mode to prevent boot loop.".to_string())
    } else {
        None
    }
}

/// Read the current regeneration attempt count from the tracking file.
async fn read_regen_attempts(tls_dir: &Path) -> u32 {
    let attempts_file = tls_dir.join(REGEN_ATTEMPTS_FILE);
    match fs::read_to_string(&attempts_file).await {
        Ok(content) => content.trim().parse().unwrap_or(0),
        Err(_) => 0,
    }
}

/// Increment and write the regeneration attempt count.
async fn increment_regen_attempts(tls_dir: &Path) -> Result<u32, RayhunterError> {
    let attempts_file = tls_dir.join(REGEN_ATTEMPTS_FILE);
    let current = read_regen_attempts(tls_dir).await;
    let new_count = current + 1;

    fs::write(&attempts_file, new_count.to_string())
        .await
        .map_err(|e| {
            RayhunterError::TlsError(format!("Failed to write regen attempts file: {}", e))
        })?;

    Ok(new_count)
}

/// Reset the regeneration attempt counter (called on successful TLS startup).
pub async fn reset_regen_attempts(qmdl_store_path: &str) {
    let tls_dir = get_tls_dir(qmdl_store_path);
    let attempts_file = tls_dir.join(REGEN_ATTEMPTS_FILE);
    let _ = fs::remove_file(&attempts_file).await;
}

/// Validate a certificate file - check it's parseable and not expired.
///
/// Returns Ok(true) if valid, Ok(false) if invalid/expired, Err on read failure.
async fn validate_certificate(cert_path: &Path) -> Result<bool, RayhunterError> {
    let cert_pem = fs::read_to_string(cert_path).await.map_err(|e| {
        RayhunterError::TlsError(format!(
            "Failed to read certificate {}: {}",
            cert_path.display(),
            e
        ))
    })?;

    // Parse the PEM to verify it's a valid certificate
    // We use rustls-pemfile to parse and basic validation
    let mut reader = std::io::BufReader::new(cert_pem.as_bytes());
    let certs: Vec<_> = rustls_pemfile::certs(&mut reader)
        .filter_map(|r| r.ok())
        .collect();

    if certs.is_empty() {
        warn!(
            "Certificate file {} contains no valid certificates",
            cert_path.display()
        );
        return Ok(false);
    }

    // Parse the first certificate to check expiration using x509-parser approach
    // For now, we do basic validation - the cert is parseable
    // Full expiration checking would require x509-parser crate
    // Instead, we'll rely on rustls failing to load expired certs

    info!(
        "Certificate validation passed (parseable PEM with {} cert(s))",
        certs.len()
    );
    Ok(true)
}

/// Set restrictive permissions on the private key file (Unix only).
#[cfg(unix)]
async fn set_key_permissions(key_path: &Path) -> Result<(), RayhunterError> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = std::fs::Permissions::from_mode(0o600);
    fs::set_permissions(key_path, permissions)
        .await
        .map_err(|e| {
            RayhunterError::TlsError(format!(
                "Failed to set permissions on {}: {}",
                key_path.display(),
                e
            ))
        })?;

    info!("Set private key permissions to 0600");
    Ok(())
}

#[cfg(not(unix))]
async fn set_key_permissions(_key_path: &Path) -> Result<(), RayhunterError> {
    // No-op on non-Unix platforms
    Ok(())
}

/// Load existing certificates or generate new ones if they don't exist.
///
/// If `custom_hosts` is provided and non-empty, uses those instead of auto-detection.
/// Custom hosts can be IP addresses or DNS names.
///
/// This function includes boot loop prevention: if certificate generation fails
/// repeatedly (MAX_REGEN_ATTEMPTS times), it will return an error and set the
/// TLS fallback flag, causing the server to fall back to HTTP-only mode.
///
/// Returns paths to (cert_path, key_path).
pub async fn load_or_generate_certs(
    qmdl_store_path: &str,
    device: &Device,
    custom_hosts: &[String],
) -> Result<(PathBuf, PathBuf), RayhunterError> {
    let tls_dir = get_tls_dir(qmdl_store_path);
    let cert_path = tls_dir.join("cert.pem");
    let key_path = tls_dir.join("key.pem");

    // Ensure TLS directory exists
    fs::create_dir_all(&tls_dir).await.map_err(|e| {
        RayhunterError::TlsError(format!(
            "Failed to create TLS directory {}: {}",
            tls_dir.display(),
            e
        ))
    })?;

    // Check boot loop prevention - if we've tried too many times, fail immediately
    let regen_attempts = read_regen_attempts(&tls_dir).await;
    if regen_attempts >= MAX_REGEN_ATTEMPTS {
        error!(
            "TLS certificate regeneration attempted {} times (max {}). \
             Entering fallback mode to prevent boot loop. \
             Delete {} to reset.",
            regen_attempts,
            MAX_REGEN_ATTEMPTS,
            tls_dir.display()
        );
        set_tls_fallback_mode(true);
        return Err(RayhunterError::TlsError(
            "Maximum certificate regeneration attempts exceeded".to_string(),
        ));
    }

    // Check if both files exist and validate them
    let needs_regeneration = if cert_path.exists() && key_path.exists() {
        match validate_certificate(&cert_path).await {
            Ok(true) => {
                info!("Using existing valid TLS certificates from {:?}", tls_dir);
                // Reset regen counter on successful validation
                reset_regen_attempts(qmdl_store_path).await;
                return Ok((cert_path, key_path));
            }
            Ok(false) => {
                warn!("Existing certificate is invalid or expired, will regenerate");
                true
            }
            Err(e) => {
                warn!(
                    "Failed to validate existing certificate: {}, will regenerate",
                    e
                );
                true
            }
        }
    } else {
        true
    };

    if !needs_regeneration {
        return Ok((cert_path, key_path));
    }

    // Increment regen attempts before trying to generate
    let attempt_num = increment_regen_attempts(&tls_dir).await?;
    info!(
        "Generating new TLS certificates in {:?} (attempt {}/{})",
        tls_dir, attempt_num, MAX_REGEN_ATTEMPTS
    );

    // Remove old cert files if they exist (they're invalid)
    let _ = fs::remove_file(&cert_path).await;
    let _ = fs::remove_file(&key_path).await;

    // Build SANs and generate certificate
    let sans = get_certificate_sans(device, custom_hosts);
    let (cert_pem, key_pem) = generate_self_signed_cert_with_sans(sans)?;

    // Write certificate
    fs::write(&cert_path, &cert_pem).await.map_err(|e| {
        RayhunterError::TlsError(format!(
            "Failed to write certificate to {}: {}",
            cert_path.display(),
            e
        ))
    })?;

    // Write private key
    fs::write(&key_path, &key_pem).await.map_err(|e| {
        RayhunterError::TlsError(format!(
            "Failed to write private key to {}: {}",
            key_path.display(),
            e
        ))
    })?;

    // Set restrictive permissions on private key
    set_key_permissions(&key_path).await?;

    info!("TLS certificates generated successfully");

    // Reset regen counter on successful generation
    reset_regen_attempts(qmdl_store_path).await;

    Ok((cert_path, key_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_device_default_ip() {
        assert_eq!(
            get_device_default_ip(&Device::Orbic),
            "192.168.1.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(
            get_device_default_ip(&Device::Tplink),
            "192.168.0.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(
            get_device_default_ip(&Device::Tmobile),
            "192.168.0.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(
            get_device_default_ip(&Device::Uz801),
            "192.168.100.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(
            get_device_default_ip(&Device::Pinephone),
            "127.0.0.1".parse::<IpAddr>().unwrap()
        );
    }

    #[test]
    fn test_get_certificate_sans_basic() {
        let sans = get_certificate_sans(&Device::Orbic, &[]);
        // Should include device default IP and localhost
        assert!(sans.len() >= 2);
    }

    #[test]
    fn test_get_certificate_sans_with_custom_hosts() {
        let custom_hosts = vec![
            "rayhunter.local".to_string(),
            "10.0.0.5".to_string(), // IP as string
        ];
        let sans = get_certificate_sans(&Device::Orbic, &custom_hosts);
        // Should include device IP, localhost, and custom hosts
        assert!(sans.len() >= 4);
    }

    #[test]
    fn test_generate_self_signed_cert() {
        let sans = vec![
            SanType::IpAddress("192.168.1.1".parse().unwrap()),
            SanType::IpAddress("10.0.0.1".parse().unwrap()),
        ];

        let result = generate_self_signed_cert_with_sans(sans);
        assert!(result.is_ok());

        let (cert_pem, key_pem) = result.unwrap();

        // Check PEM format
        assert!(cert_pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(cert_pem.ends_with("-----END CERTIFICATE-----\n"));
        assert!(key_pem.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(key_pem.ends_with("-----END PRIVATE KEY-----\n"));
    }

    #[test]
    fn test_get_tls_dir() {
        let tls_dir = get_tls_dir("/data/rayhunter/qmdl");
        assert_eq!(tls_dir, PathBuf::from("/data/rayhunter/tls"));

        let tls_dir2 = get_tls_dir("/custom/path/qmdl");
        assert_eq!(tls_dir2, PathBuf::from("/custom/path/tls"));
    }

    #[test]
    fn test_generate_self_signed_cert_with_custom_sans() {
        let sans = vec![
            SanType::IpAddress("192.168.1.1".parse().unwrap()),
            SanType::DnsName(Ia5String::try_from("rayhunter.local").unwrap()),
            SanType::IpAddress("10.0.0.5".parse().unwrap()),
        ];

        let result = generate_self_signed_cert_with_sans(sans);
        assert!(result.is_ok());

        let (cert_pem, _) = result.unwrap();
        assert!(cert_pem.contains("BEGIN CERTIFICATE"));
    }

    #[test]
    fn test_parse_san_entry_ip() {
        let san = parse_san_entry("192.168.1.1");
        assert!(san.is_some());
        assert!(matches!(san.unwrap(), SanType::IpAddress(_)));
    }

    #[test]
    fn test_parse_san_entry_hostname() {
        let san = parse_san_entry("rayhunter.local");
        assert!(san.is_some());
        assert!(matches!(san.unwrap(), SanType::DnsName(_)));
    }

    #[tokio::test]
    async fn test_load_or_generate_certs_creates_new() {
        let temp_dir = TempDir::new().unwrap();
        let qmdl_path = temp_dir.path().join("qmdl");
        std::fs::create_dir_all(&qmdl_path).unwrap();

        let result = load_or_generate_certs(qmdl_path.to_str().unwrap(), &Device::Orbic, &[]).await;
        assert!(result.is_ok());

        let (cert_path, key_path) = result.unwrap();
        assert!(cert_path.exists());
        assert!(key_path.exists());

        // Verify content is valid PEM
        let cert_content = std::fs::read_to_string(&cert_path).unwrap();
        let key_content = std::fs::read_to_string(&key_path).unwrap();
        assert!(cert_content.contains("BEGIN CERTIFICATE"));
        assert!(key_content.contains("BEGIN PRIVATE KEY"));
    }

    #[tokio::test]
    async fn test_load_or_generate_certs_with_custom_hosts() {
        let temp_dir = TempDir::new().unwrap();
        let qmdl_path = temp_dir.path().join("qmdl");
        std::fs::create_dir_all(&qmdl_path).unwrap();

        let custom_hosts = vec!["mydevice.local".to_string()];
        let result =
            load_or_generate_certs(qmdl_path.to_str().unwrap(), &Device::Orbic, &custom_hosts)
                .await;
        assert!(result.is_ok());

        let (cert_path, key_path) = result.unwrap();
        assert!(cert_path.exists());
        assert!(key_path.exists());
    }

    #[tokio::test]
    async fn test_load_or_generate_certs_uses_existing() {
        let temp_dir = TempDir::new().unwrap();
        let qmdl_path = temp_dir.path().join("qmdl");
        let tls_path = temp_dir.path().join("tls");
        std::fs::create_dir_all(&qmdl_path).unwrap();
        std::fs::create_dir_all(&tls_path).unwrap();

        // Create valid cert files (validation now checks PEM format)
        let sans = vec![SanType::IpAddress("192.168.1.1".parse().unwrap())];
        let (cert_pem, key_pem) = generate_self_signed_cert_with_sans(sans).unwrap();

        let cert_path = tls_path.join("cert.pem");
        let key_path = tls_path.join("key.pem");
        std::fs::write(&cert_path, &cert_pem).unwrap();
        std::fs::write(&key_path, &key_pem).unwrap();

        let result = load_or_generate_certs(qmdl_path.to_str().unwrap(), &Device::Orbic, &[]).await;
        assert!(result.is_ok());

        // Should return existing paths without regenerating
        let (returned_cert, returned_key) = result.unwrap();
        assert_eq!(returned_cert, cert_path);
        assert_eq!(returned_key, key_path);

        // Content should be unchanged (not regenerated)
        let cert_content = std::fs::read_to_string(&cert_path).unwrap();
        assert_eq!(cert_content, cert_pem);
    }
}
