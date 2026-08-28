//! The "packed BCD" PLMN encoding, used by 3GPP NAS IEs (`TAI`, `LAI`) and by
//! USIM elementary files (EF_HPLMNwAcT, EF_EHPLMN, EF_FPLMN).
//!
//! TS 31.102 §4.2.54 defines the PLMN in EF_HPLMNwAcT as coded "according to
//! TS 24.008". The nibble order below is easiest to check against pysim's:
//! <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/utils.py#L150-L183>
//!
//! ```text
//! byte0: [MCC2][MCC1]
//! byte1: [MNC3][MCC3]     MNC3 == 0xF when the MNC is 2 digits
//! byte2: [MNC2][MNC1]
//! ```

pub const PACKED_BCD_LEN: usize = 3;

/// Decode a 3-byte packed-BCD PLMN into an `"MCC-MNC"` string.
///
/// `None` for a wrong-length slice, an unused (all-`0xFF`) list slot, or a
/// non-numeric digit.
pub fn decode_packed_bcd(bytes: &[u8]) -> Option<String> {
    let [b0, b1, b2] = bytes else {
        return None;
    };

    // Unused entries in USIM PLMN lists are padded with 0xFF.
    // <https://github.com/osmocom/pysim/blob/25e43e1540144be9026a2733bc3a4271b8fa7d25/pySim/ts_51_011.py#L868-L869>
    if *b0 == 0xFF && *b1 == 0xFF && *b2 == 0xFF {
        return None;
    }

    let mcc_digits = [b0 & 0x0F, (b0 >> 4) & 0x0F, b1 & 0x0F];
    let mnc_digit_3 = (b1 >> 4) & 0x0F;
    let mnc_digits_1_2 = [b2 & 0x0F, (b2 >> 4) & 0x0F];

    // The MCC is always three digits, none of which may be the filler.
    if mcc_digits.iter().any(|d| *d > 9) || mnc_digits_1_2.iter().any(|d| *d > 9) {
        return None;
    }

    let mut out = String::with_capacity(7);
    for d in mcc_digits {
        out.push((b'0' + d) as char);
    }
    out.push('-');
    for d in mnc_digits_1_2 {
        out.push((b'0' + d) as char);
    }
    match mnc_digit_3 {
        0xF => {}
        d if d <= 9 => out.push((b'0' + d) as char),
        _ => return None,
    }

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_two_digit_mnc() {
        assert_eq!(
            decode_packed_bcd(&[0x32, 0xF2, 0x30]),
            Some("232-03".to_string())
        );
    }

    #[test]
    fn decodes_three_digit_mnc() {
        assert_eq!(
            decode_packed_bcd(&[0x13, 0x00, 0x62]),
            Some("310-260".to_string())
        );
    }

    #[test]
    fn two_and_three_digit_mncs_are_distinguishable() {
        let two = decode_packed_bcd(&[0x32, 0xF2, 0x30]).unwrap();
        let three = decode_packed_bcd(&[0x32, 0x02, 0x30]).unwrap();
        assert_eq!(two, "232-03");
        assert_eq!(three, "232-030");
        assert_ne!(two, three);
    }

    #[test]
    fn rejects_unused_padding_entry() {
        assert_eq!(decode_packed_bcd(&[0xFF, 0xFF, 0xFF]), None);
    }

    #[test]
    fn rejects_wrong_length() {
        assert_eq!(decode_packed_bcd(&[]), None);
        assert_eq!(decode_packed_bcd(&[0x32, 0xF2]), None);
        assert_eq!(decode_packed_bcd(&[0x32, 0xF2, 0x30, 0x00]), None);
    }

    #[test]
    fn rejects_filler_in_mcc() {
        assert_eq!(decode_packed_bcd(&[0xF2, 0xF2, 0x30]), None);
        assert_eq!(decode_packed_bcd(&[0x32, 0xFF, 0x30]), None);
    }

    #[test]
    fn rejects_filler_in_first_two_mnc_digits() {
        assert_eq!(decode_packed_bcd(&[0x32, 0xF2, 0x3F]), None);
    }
}
