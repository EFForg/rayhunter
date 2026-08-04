use std::time::Duration;

use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

use crate::plmn::{PACKED_BCD_LEN, decode_packed_bcd};
use crate::sim::SimError;

/// Ignored by SMD, but required to open the port.
const BAUD_RATE: u32 = 115_200;

const TIMEOUT: Duration = Duration::from_secs(3);

/// READ BINARY (176) of EF_HPLMNwAcT (28514 = `0x6F62`), 10 bytes.
const READ_EF_HPLMNWACT: &str = "AT+CRSM=176,28514,0,0,10";

/// 3-byte packed-BCD PLMN plus a 2-byte Access Technology identifier.
const RECORD_LEN: usize = 5;

pub async fn get_home_plmn(port: &str) -> Result<String, SimError> {
    let response = send_at_command(port, READ_EF_HPLMNWACT).await?;
    let (sw1, sw2, payload) = parse_crsm_response(&response)?;

    // 0x9000 is success; 0x91/0x9E/0x9F mean success with extra data available.
    if !matches!(sw1, 0x90 | 0x91 | 0x9E | 0x9F) {
        return Err(SimError::AtCommandError(format!(
            "reading EF_HPLMNwAcT failed with status {sw1},{sw2}"
        )));
    }

    parse_hplmnwact(&payload).ok_or_else(|| {
        SimError::AtCommandError("EF_HPLMNwAcT contained no usable PLMN".to_string())
    })
}

/// Send one AT command and read until a terminating status line. Command echo
/// is left enabled, so parse by line, not by position.
async fn send_at_command(port: &str, command: &str) -> Result<String, SimError> {
    let at = async {
        let mut serial = tokio_serial::new(port, BAUD_RATE)
            .open_native_async()
            .map_err(|e| SimError::AtCommandError(format!("couldn't open {port}: {e}")))?;

        serial.write_all(format!("{command}\r").as_bytes()).await?;

        let mut response = String::new();
        let mut chunk = [0u8; 512];
        while !is_terminated(&response) {
            let n = serial.read(&mut chunk).await?;
            if n == 0 {
                break;
            }
            response.push_str(&String::from_utf8_lossy(&chunk[..n]));
        }
        Ok::<_, SimError>(response)
    };

    let response = tokio::time::timeout(TIMEOUT, at).await.map_err(|_| {
        SimError::AtCommandError(format!("timed out waiting for a response to {command:?}"))
    })??;

    debug!("AT command {command:?} -> {response:?}");
    Ok(response)
}

/// Whether a response buffer has reached a terminating status line.
fn is_terminated(response: &str) -> bool {
    response.lines().map(str::trim).any(|line| {
        line == "OK"
            || line == "ERROR"
            || line.starts_with("+CME ERROR:")
            || line.starts_with("+CMS ERROR:")
    })
}

/// Pull `sw1`, `sw2` and the payload out of a `+CRSM: <sw1>,<sw2>[,"<hex>"]`
/// line. The payload is absent on failure statuses.
fn parse_crsm_response(response: &str) -> Result<(u8, u8, Vec<u8>), SimError> {
    let line = response
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("+CRSM:"))
        .ok_or_else(|| {
            SimError::AtCommandError(format!("no +CRSM line in response {response:?}"))
        })?;

    let mut fields = line.trim_start_matches("+CRSM:").trim().split(',');
    let mut next_u8 = |what: &str| -> Result<u8, SimError> {
        fields
            .next()
            .map(str::trim)
            .and_then(|f| f.parse::<u8>().ok())
            .ok_or_else(|| SimError::AtCommandError(format!("couldn't parse {what} from {line:?}")))
    };
    let sw1 = next_u8("sw1")?;
    let sw2 = next_u8("sw2")?;

    let payload = match fields.next() {
        Some(field) => decode_hex(field.trim().trim_matches('"')).ok_or_else(|| {
            SimError::AtCommandError(format!("couldn't decode hex payload in {line:?}"))
        })?,
        None => Vec::new(),
    };

    Ok((sw1, sw2, payload))
}

/// Decode a hex string, ignoring any whitespace within it.
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    let digits: Vec<u8> = s
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .map(|b| (b as char).to_digit(16).map(|d| d as u8))
        .collect::<Option<Vec<u8>>>()?;

    if !digits.len().is_multiple_of(2) {
        return None;
    }
    Some(digits.chunks(2).map(|c| (c[0] << 4) | c[1]).collect())
}

/// Pull the first usable PLMN out of an EF_HPLMNwAcT payload. Further records
/// are dropped (usually the same PLMN on another access technology).
fn parse_hplmnwact(payload: &[u8]) -> Option<String> {
    payload
        .chunks(RECORD_LEN)
        .filter(|record| record.len() >= PACKED_BCD_LEN)
        .find_map(|record| decode_packed_bcd(&record[..PACKED_BCD_LEN]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_terminators() {
        assert!(is_terminated("\r\nOK\r\n"));
        assert!(is_terminated("\r\nERROR\r\n"));
        assert!(is_terminated("\r\n+CME ERROR: unknown\r\n"));
        assert!(!is_terminated("\r\n+CRSM: 144,0,\"32F230"));
        assert!(!is_terminated(""));
    }

    #[test]
    fn parses_successful_crsm_response() {
        // Real response from a TP-Link M7350.
        let raw =
            "AT+CRSM=176,28514,0,0,10\r\n\r\n+CRSM: 144,0,\"32F230400032F2308000\"\r\n\r\nOK\r\n";
        let (sw1, sw2, payload) = parse_crsm_response(raw).unwrap();
        assert_eq!((sw1, sw2), (144, 0));
        assert_eq!(
            payload,
            vec![0x32, 0xF2, 0x30, 0x40, 0x00, 0x32, 0xF2, 0x30, 0x80, 0x00]
        );
    }

    #[test]
    fn parses_file_not_found_without_payload() {
        let (sw1, sw2, payload) = parse_crsm_response("\r\n+CRSM: 106,130\r\n\r\nOK\r\n").unwrap();
        assert_eq!((sw1, sw2), (106, 130));
        assert!(payload.is_empty());
    }

    #[test]
    fn errors_when_no_crsm_line_present() {
        assert!(parse_crsm_response("\r\n+CME ERROR: unknown\r\n").is_err());
    }

    #[test]
    fn rejects_malformed_hex() {
        assert_eq!(decode_hex("32F"), None);
        assert_eq!(decode_hex("32FZ"), None);
    }

    #[test]
    fn parses_real_tplink_payload() {
        // 232-03 on E-UTRAN (0x4000), then the same PLMN on UTRAN (0x8000).
        let payload = [0x32, 0xF2, 0x30, 0x40, 0x00, 0x32, 0xF2, 0x30, 0x80, 0x00];
        assert_eq!(parse_hplmnwact(&payload), Some("232-03".to_string()));
    }

    #[test]
    fn parses_real_orbic_payload() {
        // 311-480 (Verizon) on E-UTRAN, then an unused record: covers a
        // 3-digit MNC and trailing 0xFF padding.
        let payload = [0x13, 0x01, 0x84, 0x40, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
        assert_eq!(parse_hplmnwact(&payload), Some("311-480".to_string()));
    }

    #[test]
    fn parses_real_orbic_response_end_to_end() {
        let raw = "\r\n+CRSM: 144,0,\"1301844000FFFFFF0000\"\r\n\r\nOK\r\n";
        let (sw1, _, payload) = parse_crsm_response(raw).unwrap();
        assert_eq!(sw1, 144);
        assert_eq!(parse_hplmnwact(&payload), Some("311-480".to_string()));
    }

    #[test]
    fn skips_unused_leading_records() {
        let payload = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x32, 0xF2, 0x30, 0x40, 0x00];
        assert_eq!(parse_hplmnwact(&payload), Some("232-03".to_string()));
    }

    #[test]
    fn handles_three_digit_mnc() {
        let payload = [0x13, 0x00, 0x62, 0x40, 0x00];
        assert_eq!(parse_hplmnwact(&payload), Some("310-260".to_string()));
    }

    #[test]
    fn returns_none_for_empty_or_padded_file() {
        assert_eq!(parse_hplmnwact(&[]), None);
        assert_eq!(parse_hplmnwact(&[0xFF; 10]), None);
    }

    #[test]
    fn ignores_trailing_partial_record() {
        let payload = [0x32, 0xF2, 0x30, 0x40, 0x00, 0xFF, 0xFF];
        assert_eq!(parse_hplmnwact(&payload), Some("232-03".to_string()));
    }
}
