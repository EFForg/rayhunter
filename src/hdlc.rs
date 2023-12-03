use crc::Crc;
use bytes::{Buf, BufMut};

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

pub fn hdlc_decapsulate(mut data: Vec<u8>, crc: &Crc<u16>) -> Vec<u8> {
    // TODO: return errors instead of panicking
    if data.len() < 3 {
        panic!("data too short to be HDLC encapsulated");
    }

    assert_eq!(data.pop(), Some(0x7e)); // ensure data ends w/ trailing character
    let mut unescaped = Vec::new();
    let mut escaping = false;
    for i in 0..data.len() {
        let b = data[i];
        if escaping {
            match b {
                0x5e => unescaped.push(0x7e),
                0x5d => unescaped.push(0x7d),
                _ => panic!("invalid HDLC escape sequence"),
            }
            escaping = false;
        } else if b == 0x7d {
            escaping = true
        } else {
            unescaped.push(b);
        }
    }

    // pop off the u16 checksum, check it against what we calculated
    let checksum_hi = unescaped.pop().unwrap();
    let checksum_lo = unescaped.pop().unwrap();
    let checksum = [checksum_lo, checksum_hi].as_slice().get_u16_le();
    assert_eq!(checksum, crc.checksum(&unescaped)); // ensure checksums match

    unescaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdlc_encapsulate() {
        let crc = Crc::<u16>::new(&crate::diag::CRC_CCITT_ALG);
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let expected = vec![1, 2, 3, 4, 145, 57, 126];
        let encapsulated = hdlc_encapsulate(data.clone(), &crc);
        assert_eq!(&encapsulated, &expected);
        assert_eq!(hdlc_decapsulate(encapsulated, &crc), data);
    }
}
