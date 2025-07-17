//! HDLC stands for "High-level Data Link Control", which the diag protocol uses
//! to encapsulate its messages. QCSuper's docs describe this in more detail
//! here:
//! <https://github.com/P1sec/QCSuper/blob/master/docs/The%20Diag%20protocol.md#the-diag-protocol-over-usb>

use bytes::Buf;
use crc::Crc;
use thiserror::Error;

use crate::diag::{
    ESCAPED_MESSAGE_ESCAPE_CHAR, ESCAPED_MESSAGE_TERMINATOR, MESSAGE_ESCAPE_CHAR,
    MESSAGE_TERMINATOR,
};

#[derive(Debug, Clone, Error, PartialEq)]
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
    let mut result: Vec<u8> = Vec::with_capacity(data.len());

    for &b in data {
        match b {
            MESSAGE_TERMINATOR => result.extend([MESSAGE_ESCAPE_CHAR, ESCAPED_MESSAGE_TERMINATOR]),
            MESSAGE_ESCAPE_CHAR => {
                result.extend([MESSAGE_ESCAPE_CHAR, ESCAPED_MESSAGE_ESCAPE_CHAR])
            }
            _ => result.push(b),
        }
    }

    for b in crc.checksum(data).to_le_bytes() {
        match b {
            MESSAGE_TERMINATOR => result.extend([MESSAGE_ESCAPE_CHAR, ESCAPED_MESSAGE_TERMINATOR]),
            MESSAGE_ESCAPE_CHAR => {
                result.extend([MESSAGE_ESCAPE_CHAR, ESCAPED_MESSAGE_ESCAPE_CHAR])
            }
            _ => result.push(b),
        }
    }

    result.push(MESSAGE_TERMINATOR);
    result
}

pub fn hdlc_decapsulate(data: &[u8], crc: &Crc<u16>) -> Result<Vec<u8>, HdlcError> {
    if data.len() < 3 {
        return Err(HdlcError::TooShort);
    }

    if data[data.len() - 1] != MESSAGE_TERMINATOR {
        return Err(HdlcError::NoTrailingCharacter(data[data.len() - 1]));
    }

    let mut unescaped = Vec::with_capacity(data.len());
    let mut escaping = false;
    for &b in &data[..data.len() - 1] {
        if escaping {
            match b {
                ESCAPED_MESSAGE_TERMINATOR => unescaped.push(MESSAGE_TERMINATOR),
                ESCAPED_MESSAGE_ESCAPE_CHAR => unescaped.push(MESSAGE_ESCAPE_CHAR),
                _ => return Err(HdlcError::InvalidEscapeSequence(b)),
            }
            escaping = false;
        } else if b == MESSAGE_ESCAPE_CHAR {
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
        return Err(HdlcError::InvalidChecksum(
            checksum,
            crc.checksum(&unescaped),
        ));
    }

    Ok(unescaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdlc_encapsulate() {
        let crc = Crc::<u16>::new(&crate::diag::CRC_CCITT_ALG);
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let expected = vec![1, 2, 3, 4, 145, 57, 126];
        let encapsulated = hdlc_encapsulate(&data, &crc);
        assert_eq!(&encapsulated, &expected);
        assert_eq!(hdlc_decapsulate(&encapsulated, &crc), Ok(data));
    }
}
