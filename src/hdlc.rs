//! HDLC stands for "High-level Data Link Control", which the diag protocol uses
//! to encapsulate its messages. QCSuper's docs describe this in more detail
//! here:
//! https://github.com/P1sec/QCSuper/blob/master/docs/The%20Diag%20protocol.md#the-diag-protocol-over-usb

use crc::Crc;
use bytes::{Buf, BufMut};
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

pub fn hdlc_encapsulate(mut data: Vec<u8>, crc: &Crc<u16>) -> Vec<u8> {
    data.put_u16_le(crc.checksum(&data));

    let mut result: Vec<u8> = data.iter()
        .flat_map(|&b| match b {
            // TODO: is this too expensive?
            0x7e => vec![0x7d, 0x5e],
            0x7d => vec![0x7d, 0x5d],
            _ => vec![b],
        })
        .collect();
    result.push(0x7e);
    result
}

pub fn hdlc_decapsulate(mut data: Vec<u8>, crc: &Crc<u16>) -> Result<Vec<u8>, HdlcError> {
    // TODO: return errors instead of panicking
    if data.len() < 3 {
        return Err(HdlcError::TooShort);
    }

    let last_char = data.pop().unwrap(); // safe since len() >= 3
    if last_char != 0x7e {
        return Err(HdlcError::NoTrailingCharacter(last_char));
    }

    let mut unescaped = Vec::new();
    let mut escaping = false;
    for i in 0..data.len() {
        let b = data[i];
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
        let crc = Crc::<u16>::new(&crate::diag_device::CRC_CCITT_ALG);
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let expected = vec![1, 2, 3, 4, 145, 57, 126];
        let encapsulated = hdlc_encapsulate(data.clone(), &crc);
        assert_eq!(&encapsulated, &expected);
        assert_eq!(hdlc_decapsulate(encapsulated, &crc), Ok(data));
    }
}
