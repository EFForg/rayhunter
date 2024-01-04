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

#[derive(Debug, PartialEq, Error)]
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
                match hdlc_decapsulate(sub_msg, &CRC_CCITT) {
                    Ok(data) => match Message::from_bytes((&data, 0)) {
                        Ok(((leftover_bytes, _), res)) => {
                            if !leftover_bytes.is_empty() {
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

#[cfg(test)]
mod test {
    use super::*;

    struct MockReader {
        containers: Vec<MessagesContainer>,
    }

    impl DiagReader for MockReader {
        type Err = ();

        fn get_next_messages_container(&mut self) -> Result<MessagesContainer, Self::Err> {
            Ok(self.containers.remove(0))
        }
    }

    fn make_container(data_type: DataType, message: HdlcEncapsulatedMessage) -> MessagesContainer {
        MessagesContainer {
            data_type,
            num_messages: 1,
            messages: vec![message],
        }
    }

    // this log is based on one captured on a real device -- if it fails to
    // serialize or deserialize, that's probably a problem with this mock, not
    // the DiagReader implementation
    fn get_test_message(payload: &[u8]) -> (HdlcEncapsulatedMessage, Message) {
        let length_with_payload = 31 + payload.len() as u16;
        let message = Message::Log {
            pending_msgs: 0,
            outer_length: length_with_payload,
            inner_length: length_with_payload,
            log_type: 0xb0c0,
            timestamp: Timestamp { ts: 72659535985485082 },
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 20,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 14,
                    rrc_rel_min: 48,
                    bearer_id: 0,
                    phy_cell_id: 160,
                    earfcn: 2050,
                    sfn_subfn: 4057,
                    pdu_num: 5,
                    sib_mask: 0,
                    len: payload.len() as u16,
                    packet: payload.to_vec(),
                },
            },
        };
        let serialized = message.to_bytes().expect("failed to serialize test message");
        let encapsulated_data = hdlc::hdlc_encapsulate(&serialized, &CRC_CCITT);
        let encapsulated = HdlcEncapsulatedMessage {
            len: encapsulated_data.len() as u32,
            data: encapsulated_data,
        };
        (encapsulated, message)
    }

    #[test]
    fn test_skipping_nonuser_containers() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, _) = get_test_message(&[2]);
        let (encapsulated3, message3) = get_test_message(&[3]);
        let mut reader = MockReader {
            containers: vec![
                make_container(DataType::UserSpace, encapsulated1),
                make_container(DataType::Other(0), encapsulated2),
                make_container(DataType::UserSpace, encapsulated3),
            ],
        };
        assert_eq!(reader.read_response(), Ok(vec![Ok(message1)]));
        assert_eq!(reader.read_response(), Ok(vec![Ok(message3)]));
    }

    #[test]
    fn test_containers_with_multiple_messages() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        let mut container1 = make_container(DataType::UserSpace, encapsulated1);
        container1.messages.push(encapsulated2);
        container1.num_messages += 1;
        let (encapsulated3, message3) = get_test_message(&[3]);
        let mut reader = MockReader {
            containers: vec![
                container1,
                make_container(DataType::UserSpace, encapsulated3),
            ],
        };
        assert_eq!(reader.read_response(), Ok(vec![Ok(message1), Ok(message2)]));
        assert_eq!(reader.read_response(), Ok(vec![Ok(message3)]));
    }

    #[test]
    fn test_containers_with_concatenated_message() {
        let (mut encapsulated1, message1) = get_test_message(&[1]);
        let (encapsulated2, message2) = get_test_message(&[2]);
        encapsulated1.data.extend(encapsulated2.data);
        encapsulated1.len += encapsulated2.len;
        let (encapsulated3, message3) = get_test_message(&[3]);
        let mut reader = MockReader {
            containers: vec![
                make_container(DataType::UserSpace, encapsulated1),
                make_container(DataType::UserSpace, encapsulated3),
            ],
        };
        assert_eq!(reader.read_response(), Ok(vec![Ok(message1), Ok(message2)]));
        assert_eq!(reader.read_response(), Ok(vec![Ok(message3)]));
    }

    #[test]
    fn test_handles_parsing_errors() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let bad_message = hdlc::hdlc_encapsulate(&[0x01, 0x02, 0x03, 0x04], &CRC_CCITT);
        let encapsulated2 = HdlcEncapsulatedMessage {
            len: bad_message.len() as u32,
            data: bad_message,
        };
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(encapsulated2);
        container.num_messages += 1;
        let mut reader = MockReader {
            containers: vec![container],
        };
        let result = reader.read_response().unwrap();
        assert_eq!(result[0], Ok(message1));
        assert!(matches!(result[1], Err(DiagParsingError::MessageParsingError(_, _))));
    }

    #[test]
    fn test_handles_encapsulation_errors() {
        let (encapsulated1, message1) = get_test_message(&[1]);
        let bad_encapsulation = HdlcEncapsulatedMessage {
            len: 4,
            data: vec![0x01, 0x02, 0x03, 0x04],
        };
        let mut container = make_container(DataType::UserSpace, encapsulated1);
        container.messages.push(bad_encapsulation);
        container.num_messages += 1;
        let mut reader = MockReader {
            containers: vec![container],
        };
        let result = reader.read_response().unwrap();
        assert_eq!(result[0], Ok(message1));
        assert!(matches!(result[1], Err(DiagParsingError::HdlcDecapsulationError(_, _))));
    }
}
