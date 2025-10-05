use anyhow::{Context, Result};
use base64_light::base64_encode;
use serde::{Deserialize, Serialize};

/// Helper function to swap characters in a string
fn swap_chars(s: &str, pos1: usize, pos2: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if pos1 < chars.len() && pos2 < chars.len() {
        chars.swap(pos1, pos2);
    }
    chars.into_iter().collect()
}

/// Apply character swapping based on secret (unchanged from original algorithm)
fn apply_secret_swapping(mut text: String, secret_num: u32) -> String {
    for i in 0..4 {
        let byte = (secret_num >> (i * 8)) & 0xff;
        let pos1 = (byte as usize) % text.len();
        let pos2 = i % text.len();
        text = swap_chars(&text, pos1, pos2);
    }
    text
}

/// Encode password using Orbic's custom algorithm
///
/// This function is a lot simpler than the original JavaScript because it always uses the same
/// character set regardless of "password type", and any randomly generated values are hardcoded.
pub fn encode_password(
    password: &str,
    secret: &str,
    timestamp: &str,
    timestamp_start: u64,
) -> Result<String> {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // MD5 hash the password and use fixed prefix "a7" instead of random chars
    let password_md5 = format!("{:x}", md5::compute(password));
    let mut spliced_password = format!("a7{}", password_md5);

    let secret_num = u32::from_str_radix(secret, 16).context("Failed to parse secret as hex")?;

    spliced_password = apply_secret_swapping(spliced_password, secret_num);

    let timestamp_hex =
        u32::from_str_radix(timestamp, 16).context("Failed to parse timestamp as hex")?;
    let time_delta = format!(
        "{:x}",
        timestamp_hex + (current_time - timestamp_start) as u32
    );

    // Use fixed hex "6137" instead of hex encoding of random values
    let message = format!("6137x{}:{}", time_delta, spliced_password);

    let result = base64_encode(&message);
    let result = apply_secret_swapping(result, secret_num);

    Ok(result)
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInfo {
    pub retcode: u32,
    #[serde(rename = "priKey")]
    pub pri_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub retcode: u32,
}
