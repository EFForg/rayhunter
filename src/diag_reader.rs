use crate::diag;
use crate::{diag::*, hdlc::hdlc_decapsulate};
use crate::hdlc;

use crc::{Crc, Algorithm};
use deku::prelude::*;
use log::{info, warn, error};
use thiserror::Error;

// this is sorta based on the params qcsuper uses, plus what seems to be used in
// https://github.com/fgsect/scat/blob/f1538b397721df3ab8ba12acd26716abcf21f78b/util.py#L47
pub const CRC_CCITT_ALG: Algorithm<u16> = Algorithm {
    poly: 0x1021,
    init: 0xffff,
    refin: true,
    refout: true,
    width: 16,
    xorout: 0xffff,
    check: 0x2189,
    residue: 0x0000,
};
pub const CRC_CCITT: Crc<u16> = Crc::<u16>::new(&CRC_CCITT_ALG);

#[derive(Debug, Error)]
pub enum DiagParsingError {
    #[error("Failed to parse Message: {0}, data: {1:?}")]
    MessageParsingError(deku::DekuError, Vec<u8>),
    #[error("HDLC decapsulation of message failed: {0}, data: {1:?}")]
    HdlcDecapsulationError(hdlc::HdlcError, Vec<u8>),
}

type MaybeMessage = Result<Message, DiagParsingError>;

pub trait DiagReader {
    type Err;

    fn get_next_messages_container(&mut self) -> Result<MessagesContainer, Self::Err>;

    fn read_response(&mut self) -> Result<Vec<MaybeMessage>, Self::Err> {
        loop {
            let container = self.get_next_messages_container()?;
            if container.data_type == DataType::UserSpace {
                return self.parse_response_container(container);
            } else {
                info!("skipping non-userspace message...")
            }
        }
    }

    fn parse_response_container(&self, container: MessagesContainer) -> Result<Vec<MaybeMessage>, Self::Err> {
        let mut result = Vec::new();
        for msg in container.messages {
            for sub_msg in msg.data.split_inclusive(|&b| b == diag::MESSAGE_TERMINATOR) {
                match hdlc_decapsulate(&sub_msg, &CRC_CCITT) {
                    Ok(data) => match Message::from_bytes((&data, 0)) {
                        Ok(((leftover_bytes, _), res)) => {
                            if leftover_bytes.len() > 0 {
                                warn!("warning: {} leftover bytes when parsing Message", leftover_bytes.len());
                            }
                            result.push(Ok(res));
                        },
                        Err(e) => {
                            result.push(Err(DiagParsingError::MessageParsingError(e, data)));
                        },
                    },
                    Err(err) => {
                        result.push(Err(DiagParsingError::HdlcDecapsulationError(err, sub_msg.to_vec())));
                    }
                }
            }
        }
        Ok(result)
    }
}
