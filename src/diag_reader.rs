use crate::diag;
use crate::{diag::*, hdlc::hdlc_decapsulate};
use crate::diag_device::DiagResult;

use crc::{Crc, Algorithm};
use deku::prelude::*;
use log::{debug, info, warn, error};

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

pub trait DiagReader {
    fn get_next_messages_container(&mut self) -> DiagResult<MessagesContainer>;

    fn read_response(&mut self) -> DiagResult<Vec<Message>> {
        loop {
            let container = self.get_next_messages_container()?;
            if container.data_type == DataType::UserSpace {
                return self.parse_response_container(container);
            } else {
                info!("skipping non-userspace message...")
            }
        }
    }

    fn parse_response_container(&self, container: MessagesContainer) -> DiagResult<Vec<Message>> {
        let mut result = Vec::new();
        for msg in container.messages {
            for sub_msg in msg.data.split_inclusive(|&b| b == diag::MESSAGE_TERMINATOR) {
                match hdlc_decapsulate(&sub_msg, &CRC_CCITT) {
                    Ok(data) => match Message::from_bytes((&data, 0)) {
                        Ok(((leftover_bytes, _), res)) => {
                            if leftover_bytes.len() > 0 {
                                warn!("warning: {} leftover bytes when parsing Message", leftover_bytes.len());
                            }
                            result.push(res);
                        },
                        Err(e) => {
                            error!("error parsing response: {:?}", e);
                            debug!("{:?}", data);
                        },
                    },
                    Err(err) => {
                        error!("error decapsulating response: {:?}", err);
                        debug!("{:?}", &sub_msg);
                    }
                }
            }
        }
        Ok(result)
    }
}
