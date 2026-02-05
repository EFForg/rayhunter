//! TLS certificate generation and management for HTTPS support.
//!
//! This module handles:
//! - Detecting device IP addresses for certificate SANs
//! - Generating self-signed certificates using rcgen
//! - Loading existing certificates or generating new ones

use std::net::IpAddr;
use std::path::{Path, PathBuf};

use log::{info, warn};
use rayhunter::Device;
use rcgen::{CertificateParams, DnType, Ia5String, KeyPair, SanType};
use tokio::fs;

use crate::error::RayhunterError;

/// Default certificate validity in days (10 years)
const CERT_VALIDITY_DAYS: u32 = 3650;

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

/// Detect IP addresses for certificate SANs.
///
/// Combines device-specific default IP with any detected network interface IPs.
/// The device default IP is always included first to ensure it's in the certificate
/// even when there's no SIM card or network connection.
pub fn detect_device_ips(device: &Device) -> Vec<IpAddr> {
    let mut ips = Vec::new();

    // Always include the device-specific default IP first
    let device_ip = get_device_default_ip(device);
    ips.push(device_ip);
    info!("Using device default IP: {}", device_ip);

    // Try to detect additional IPs from network interfaces
    match if_addrs::get_if_addrs() {
        Ok(interfaces) => {
            let detected: Vec<IpAddr> = interfaces
                .into_iter()
                .filter(|iface| !iface.is_loopback())
                .map(|iface| iface.ip())
                .filter(|ip| ip.is_ipv4() && !ips.contains(ip))
                .collect();

            if !detected.is_empty() {
                info!("Also detected network IPs: {:?}", detected);
                ips.extend(detected);
            }
        }
        Err(e) => {
            warn!("Failed to detect network interfaces: {}", e);
        }
    }

    ips
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

/// Generate a self-signed certificate with the given IP addresses as SANs.
///
/// Returns (certificate_pem, private_key_pem) as strings.
#[allow(dead_code)] // Used by tests and kept as simpler public API
pub fn generate_self_signed_cert(ips: &[IpAddr]) -> Result<(String, String), RayhunterError> {
    generate_self_signed_cert_with_hosts(ips, &[])
}

/// Generate a self-signed certificate with IPs and custom hosts as SANs.
///
/// Custom hosts can be IP addresses or DNS names (hostnames).
/// Returns (certificate_pem, private_key_pem) as strings.
pub fn generate_self_signed_cert_with_hosts(
    ips: &[IpAddr],
    custom_hosts: &[String],
) -> Result<(String, String), RayhunterError> {
    let mut params = CertificateParams::default();

    // Set certificate subject
    params
        .distinguished_name
        .push(DnType::CommonName, "Rayhunter");
    params
        .distinguished_name
        .push(DnType::OrganizationName, "Electronic Frontier Foundation");

    // Add Subject Alternative Names (SANs)
    let mut sans: Vec<SanType> = ips.iter().map(|ip| SanType::IpAddress(*ip)).collect();

    // Add custom hosts (can be IPs or DNS names)
    for host in custom_hosts {
        let host = host.trim();
        if host.is_empty() {
            continue;
        }
        if let Some(san) = parse_san_entry(host) {
            // Avoid duplicates
            if !sans.contains(&san) {
                info!("Adding custom SAN: {}", host);
                sans.push(san);
            }
        }
    }

    // Always include localhost
    let localhost_san = SanType::IpAddress("127.0.0.1".parse().unwrap());
    if !sans.contains(&localhost_san) {
        sans.push(localhost_san);
    }

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

/// Load existing certificates or generate new ones if they don't exist.
///
/// If `custom_hosts` is provided and non-empty, uses those instead of auto-detection.
/// Custom hosts can be IP addresses or DNS names.
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

    // Check if both files exist
    if cert_path.exists() && key_path.exists() {
        info!("Using existing TLS certificates from {:?}", tls_dir);
        return Ok((cert_path, key_path));
    }

    // Generate new certificates
    info!("Generating new TLS certificates in {:?}", tls_dir);

    // Ensure TLS directory exists
    fs::create_dir_all(&tls_dir)
        .await
        .map_err(|e| RayhunterError::TlsError(format!("Failed to create TLS directory: {}", e)))?;

    // Detect IPs (using device-specific defaults) and generate cert with custom hosts
    let ips = detect_device_ips(device);
    let (cert_pem, key_pem) = generate_self_signed_cert_with_hosts(&ips, custom_hosts)?;

    // Write certificate and key
    fs::write(&cert_path, &cert_pem)
        .await
        .map_err(|e| RayhunterError::TlsError(format!("Failed to write certificate: {}", e)))?;

    fs::write(&key_path, &key_pem)
        .await
        .map_err(|e| RayhunterError::TlsError(format!("Failed to write private key: {}", e)))?;

    info!("TLS certificates generated successfully");
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
    fn test_detect_device_ips_orbic() {
        let ips = detect_device_ips(&Device::Orbic);
        // Should always include the device default IP first
        assert!(!ips.is_empty());
        assert_eq!(ips[0], "192.168.1.1".parse::<IpAddr>().unwrap());
        // All should be valid IPv4 addresses
        for ip in &ips {
            assert!(ip.is_ipv4());
        }
    }

    #[test]
    fn test_detect_device_ips_tplink() {
        let ips = detect_device_ips(&Device::Tplink);
        assert!(!ips.is_empty());
        assert_eq!(ips[0], "192.168.0.1".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn test_generate_self_signed_cert() {
        let ips: Vec<IpAddr> = vec!["192.168.1.1".parse().unwrap(), "10.0.0.1".parse().unwrap()];

        let result = generate_self_signed_cert(&ips);
        assert!(result.is_ok());

        let (cert_pem, key_pem) = result.unwrap();

        // Check PEM format
        assert!(cert_pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(cert_pem.ends_with("-----END CERTIFICATE-----\n"));
        assert!(key_pem.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(key_pem.ends_with("-----END PRIVATE KEY-----\n"));
    }

    #[test]
    fn test_generate_self_signed_cert_empty_ips() {
        // Should still work with empty IPs (will add localhost)
        let result = generate_self_signed_cert(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_tls_dir() {
        let tls_dir = get_tls_dir("/data/rayhunter/qmdl");
        assert_eq!(tls_dir, PathBuf::from("/data/rayhunter/tls"));

        let tls_dir2 = get_tls_dir("/custom/path/qmdl");
        assert_eq!(tls_dir2, PathBuf::from("/custom/path/tls"));
    }

    #[test]
    fn test_generate_self_signed_cert_with_custom_hosts() {
        let ips: Vec<IpAddr> = vec!["192.168.1.1".parse().unwrap()];
        let custom_hosts = vec![
            "rayhunter.local".to_string(),
            "10.0.0.5".to_string(), // IP as string
        ];

        let result = generate_self_signed_cert_with_hosts(&ips, &custom_hosts);
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

        // Create dummy cert files
        let cert_path = tls_path.join("cert.pem");
        let key_path = tls_path.join("key.pem");
        std::fs::write(&cert_path, "existing cert").unwrap();
        std::fs::write(&key_path, "existing key").unwrap();

        let result = load_or_generate_certs(qmdl_path.to_str().unwrap(), &Device::Orbic, &[]).await;
        assert!(result.is_ok());

        // Should return existing paths without modifying
        let (returned_cert, returned_key) = result.unwrap();
        assert_eq!(returned_cert, cert_path);
        assert_eq!(returned_key, key_path);

        // Content should be unchanged
        let cert_content = std::fs::read_to_string(&cert_path).unwrap();
        assert_eq!(cert_content, "existing cert");
    }
}
