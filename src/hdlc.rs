//! HDLC stands for "High-level Data Link Control", which the diag protocol uses
//! to encapsulate its messages. QCSuper's docs describe this in more detail
//! here:
//! https://github.com/P1sec/QCSuper/blob/master/docs/The%20Diag%20protocol.md#the-diag-protocol-over-usb

use crc::Crc;
use bytes::Buf;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum HdlcError {
    #[error("Invalid checksum (expected {0}, got {1})")]
    InvalidChecksum(u16, u16),
    #[error("Invalid HDLC escape sequence: [0x7d, {0}]")]
    InvalidEscapeSequence(u8),
    #[error("No trailing character found (expected 0x7e, got {0}))")]
    NoTrailingCharacter(u8),
    #[error("Missing checksum")]
    MissingChecksum,
    #[error("Data too short to be HDLC encapsulated")]
    TooShort,
}

pub fn hdlc_encapsulate(data: &[u8], crc: &Crc<u16>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];

    for &b in data {
        match b {
            0x7e => result.extend([0x7d, 0x5e]),
            0x7d => result.extend([0x7d, 0x5d]),
            _ => result.push(b),
        }
    }

    for b in crc.checksum(&data).to_le_bytes() {
        match b {
            0x7e => result.extend([0x7d, 0x5e]),
            0x7d => result.extend([0x7d, 0x5d]),
            _ => result.push(b),
        }
    }

    result.push(0x7e);
    result
}

pub fn hdlc_decapsulate(data: &[u8], crc: &Crc<u16>) -> Result<Vec<u8>, HdlcError> {
    if data.len() < 3 {
        return Err(HdlcError::TooShort);
    }

    if data[data.len() - 1] != 0x7e {
        return Err(HdlcError::NoTrailingCharacter(data[data.len() - 1]));
    }

    let mut unescaped = Vec::new();
    let mut escaping = false;
    for &b in &data[..data.len() - 1] {
        if escaping {
            match b {
                0x5e => unescaped.push(0x7e),
                0x5d => unescaped.push(0x7d),
                _ => return Err(HdlcError::InvalidEscapeSequence(b)),
            }
            escaping = false;
        } else if b == 0x7d {
            escaping = true
        } else {
            unescaped.push(b);
        }
    }

    // pop off the u16 checksum, check it against what we calculated
    let checksum_hi = unescaped.pop().ok_or(HdlcError::MissingChecksum)?;
    let checksum_lo = unescaped.pop().ok_or(HdlcError::MissingChecksum)?;
    let checksum = [checksum_lo, checksum_hi].as_slice().get_u16_le();
    if checksum != crc.checksum(&unescaped) {
        return Err(HdlcError::InvalidChecksum(checksum, crc.checksum(&unescaped)));
    }

    Ok(unescaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdlc_encapsulate() {
        let crc = Crc::<u16>::new(&crate::diag_reader::CRC_CCITT_ALG);
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let expected = vec![1, 2, 3, 4, 145, 57, 126];
        let encapsulated = hdlc_encapsulate(&data, &crc);
        assert_eq!(&encapsulated, &expected);
        assert_eq!(hdlc_decapsulate(&encapsulated, &crc), Ok(data));
    }
}
