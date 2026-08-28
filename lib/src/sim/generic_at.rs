//! Reading the subscriber's home PLMN list off the SIM over a serial AT
//! command interface.

use std::collections::BTreeSet;
use std::time::Duration;

use log::debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

use crate::plmn::{PACKED_BCD_LEN, decode_packed_bcd};
use crate::sim::SimError;

/// Required to open the port. Unverified whether SMD honours it at all.
const BAUD_RATE: u32 = 115_200;

const TIMEOUT: Duration = Duration::from_secs(3);

/// Ask the modem to read the first two records of EF_HPLMNwAcT off the SIM.
///
/// EF_HPLMNwAcT is the file whose "data field shall contain the HPLMN code, or
/// codes together with the respected access technology" -- that is, the
/// operator(s) the card considers to be its own home network. TS 31.102 §4.2.54 gives
/// "Identifier: '6F62'", "Structure: Transparent" and "File size: 5n (n >= 1)
/// bytes".
///
/// Syntax is table 78 of 3GPP TS 27.007 §8.18, p. 138 of
/// <https://www.etsi.org/deliver/etsi_ts/127000_127099/127007/19.06.00_60/ts_127007v190600p.pdf>:
///     +CRSM=<command>,<fileid>,<P1>,<P2>,<P3>
/// where:
/// * `command` = `176`, or READ BINARY
/// * `fileid` = `28514`, or `0x6F62`, which is `EFHPLMNwAcT`'s file ID
/// * `<P1>,<P2>,<P3>` = `0,0,10`, or offset 0, read 10 bytes, i.e. the first two 5-byte records.
///
///   If a nonzero offset is needed, refer to TS 102 221 §11.1.3.2:
///   <https://www.etsi.org/deliver/etsi_ts/102200_102299/102221/18.03.00_60/ts_102221v180300p.pdf>
///
/// Two records is arbitrary. The file is "5n (n >= 1) bytes" with no upper
/// bound and we never tried reading more.
const READ_EF_HPLMNWACT: &str = "AT+CRSM=176,28514,0,0,10";

const RECORD_LEN: usize = 5;

pub async fn get_home_plmn(port: &str) -> Result<BTreeSet<String>, SimError> {
    let response = send_at_command(port, READ_EF_HPLMNWACT).await?;
    let (sw1, sw2, payload) = parse_crsm_response(&response)?;

    if !is_normal_ending(sw1, sw2) {
        return Err(SimError::AtCommandError(format!(
            "reading EF_HPLMNwAcT failed with status {sw1},{sw2}"
        )));
    }

    let plmns = parse_hplmnwact(&payload);
    if plmns.is_empty() {
        return Err(SimError::AtCommandError(
            "EF_HPLMNwAcT contained no usable PLMN".to_string(),
        ));
    }
    Ok(plmns)
}

/// Table 10.7 ("Status byte coding - normal processing") of ETSI TS 102 221
/// V18.3.0 §10.2.1.1 lists exactly three normal endings, and this matches them:
/// <https://www.etsi.org/deliver/etsi_ts/102200_102299/102221/18.03.00_60/ts_102221v180300p.pdf>
fn is_normal_ending(sw1: u8, sw2: u8) -> bool {
    matches!((sw1, sw2), (0x90, 0x00) | (0x91 | 0x92, _))
}

/// Send one AT command and read until a terminating status line.
///
/// `\r` terminates the command line. Command echo is left enabled, so parse by
/// line, not by position. pysim likewise leaves the modem's echo setting alone
/// and detects it instead:
/// <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/transport/modem_atcmd.py#L108-L124>
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

/// Whether a response buffer has reached a final result code. Same set pysim
/// terminates on:
/// <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/transport/modem_atcmd.py#L80-L88>
///
/// `+CME ERROR: <err>` is "similar to the regular ERROR result code" and is what
/// `+CRSM` returns "when the command cannot be passed to the SIM" (3GPP TS 27.007
/// §9.2 and §8.18). `+CMS ERROR:` is the SMS-flavoured equivalent from TS 27.005
/// and shouldn't turn up here, but firmware is careless about error namespaces
/// and treating it as terminal beats waiting out the timeout.
fn is_terminated(response: &str) -> bool {
    response.lines().map(str::trim).any(|line| {
        line == "OK"
            || line == "ERROR"
            || line.starts_with("+CME ERROR:")
            || line.starts_with("+CMS ERROR:")
    })
}

/// Parse `+CRSM: <sw1>,<sw2>[,<response>]`, per table 78 of 3GPP TS 27.007
/// §8.18 ("Restricted SIM access +CRSM"), p. 138 of
/// <https://www.etsi.org/deliver/etsi_ts/127000_127099/127007/19.06.00_60/ts_127007v190600p.pdf>.
///
/// `<sw1>`/`<sw2>` are a pair of status codes that have to be checked together,
/// see [`is_normal_ending`]. They're always present; `<response>` is not, since
/// it's the "response of a successful completion of the command previously
/// issued". Both are "integer type", hence decimal parsing even though every
/// status word table is written in hex (`144,0` is `0x90,0x00`).
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

/// Pull every usable PLMN out of an EF_HPLMNwAcT payload: a flat array of
/// 5-byte records, decoded like pysim's `EF_xPLMNwAcT`, including the all-`0xFF`
/// skip:
/// <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/ts_51_011.py#L868-L872>
///
/// The AcT mask is dropped. Bit 15 is UTRAN, while E-UTRAN is a 3-bit field at
/// bits 14-12 (`0x7000`):
/// <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/utils.py#L186-L207>
///
/// So one operator can appear twice, as `..8000` (UTRAN) and `..4000` (E-UTRAN
/// WB-S1 + NB-S1); the set collapses those. Trailing partial records are
/// ignored.
fn parse_hplmnwact(payload: &[u8]) -> BTreeSet<String> {
    payload
        .chunks(RECORD_LEN)
        .filter_map(|record| decode_packed_bcd(record.get(..PACKED_BCD_LEN)?))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(plmns: &[&str]) -> BTreeSet<String> {
        plmns.iter().map(|p| p.to_string()).collect()
    }

    #[test]
    fn detects_terminators() {
        assert!(is_terminated("\r\nOK\r\n"));
        assert!(is_terminated("\r\nERROR\r\n"));
        assert!(is_terminated("\r\n+CME ERROR: unknown\r\n"));
        assert!(!is_terminated("\r\n+CRSM: 144,0,\"32F230"));
        assert!(!is_terminated(""));
    }

    #[test]
    fn accepts_only_normal_ending_status_words() {
        // TS 102 221 table 10.7, normal processing.
        assert!(is_normal_ending(0x90, 0x00));
        assert!(is_normal_ending(0x91, 0x20));
        assert!(is_normal_ending(0x92, 0x01));

        // 0x90 is only a success paired with sw2 == 0x00.
        assert!(!is_normal_ending(0x90, 0x04));

        // GSM-only codes: +CRSM gives us the payload inline, so treating these
        // as success would mean silently parsing data we never fetched.
        assert!(!is_normal_ending(0x9E, 0x0A));
        assert!(!is_normal_ending(0x9F, 0x0A));

        // "File not found" and friends.
        assert!(!is_normal_ending(0x6A, 0x82));
        assert!(!is_normal_ending(0x6F, 0x00));
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
        assert_eq!(parse_hplmnwact(&payload), set(&["232-03"]));
    }

    #[test]
    fn parses_real_orbic_payload() {
        // 311-480 (Verizon) on E-UTRAN, then an unused record: covers a
        // 3-digit MNC and trailing 0xFF padding.
        let payload = [0x13, 0x01, 0x84, 0x40, 0x00, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
        assert_eq!(parse_hplmnwact(&payload), set(&["311-480"]));
    }

    #[test]
    fn parses_real_orbic_response_end_to_end() {
        let raw = "\r\n+CRSM: 144,0,\"1301844000FFFFFF0000\"\r\n\r\nOK\r\n";
        let (sw1, _, payload) = parse_crsm_response(raw).unwrap();
        assert_eq!(sw1, 144);
        assert_eq!(parse_hplmnwact(&payload), set(&["311-480"]));
    }

    #[test]
    fn skips_unused_leading_records() {
        let payload = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x32, 0xF2, 0x30, 0x40, 0x00];
        assert_eq!(parse_hplmnwact(&payload), set(&["232-03"]));
    }

    #[test]
    fn handles_three_digit_mnc() {
        let payload = [0x13, 0x00, 0x62, 0x40, 0x00];
        assert_eq!(parse_hplmnwact(&payload), set(&["310-260"]));
    }

    #[test]
    fn returns_none_for_empty_or_padded_file() {
        assert!(parse_hplmnwact(&[]).is_empty());
        assert!(parse_hplmnwact(&[0xFF; 10]).is_empty());
    }

    #[test]
    fn ignores_trailing_partial_record() {
        let payload = [0x32, 0xF2, 0x30, 0x40, 0x00, 0xFF, 0xFF];
        assert_eq!(parse_hplmnwact(&payload), set(&["232-03"]));
    }

    #[test]
    fn keeps_distinct_plmns() {
        // 232-03 on E-UTRAN, then 232-07 on UTRAN.
        let payload = [0x32, 0xF2, 0x30, 0x40, 0x00, 0x32, 0xF2, 0x70, 0x80, 0x00];
        assert_eq!(parse_hplmnwact(&payload), set(&["232-03", "232-07"]));
    }
}
