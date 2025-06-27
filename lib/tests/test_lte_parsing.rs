use deku::prelude::*;
use rayhunter::{
    diag::{LogBody, LteRrcOtaPacket, Message, Timestamp},
    gsmtap_parser,
};

// Tests here are based on https://github.com/fgsect/scat/blob/97442580e628de414c9f7c2a185f4e28d0ee7523/tests/test_diagltelogparser.py

#[test]
fn test_lte_rrc_ota() {
    let v26_binary = &[
        0x10, 0x0, 0x23, 0x0, 0x23, 0x0, 0xc0, 0xb0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a,
        0xf, 0x40, 0xf, 0x40, 0x1, 0xe, 0x1, 0x13, 0x7, 0x0, 0x0, 0x0, 0x0, 0xb, 0x0, 0x0, 0x0,
        0x0, 0x2, 0x0, 0x10, 0x15,
    ];
    let (_, parsed) = Message::from_bytes((v26_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 0x23,
            inner_length: 0x23,
            timestamp: Timestamp { ts: 0 },
            log_type: 0xb0c0,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 26,
                packet: LteRrcOtaPacket::V25 {
                    rrc_rel_maj: 15,
                    rrc_rel_min: 64,
                    nr_rrc_rel_maj: 15,
                    nr_rrc_rel_min: 64,
                    bearer_id: 1,
                    phy_cell_id: 270,
                    earfcn: 1811,
                    sfn_subfn: 0,
                    pdu_num: 11,
                    sib_mask: 0,
                    len: 2,
                    packet: vec![0x10, 0x15],
                }
            }
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(&gsmtap_msg.payload, &[0x10, 0x15]);
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1811);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 3);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v26_binary = &[
        0x10, 0x00, 0x23, 0x00, 0x23, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x1a, 0x0f, 0x40, 0x0f, 0x40, 0x01, 0x0e, 0x01, 0x13, 0x07, 0x00, 0x00, 0x00, 0x00,
        0x0b, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x10, 0x15,
    ];
    let (_, parsed) = Message::from_bytes((v26_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 35,
            inner_length: 35,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 26,
                packet: LteRrcOtaPacket::V25 {
                    rrc_rel_maj: 15,
                    rrc_rel_min: 64,
                    nr_rrc_rel_maj: 15,
                    nr_rrc_rel_min: 64,
                    bearer_id: 1,
                    phy_cell_id: 270,
                    earfcn: 1811,
                    sfn_subfn: 0,
                    pdu_num: 11,
                    sib_mask: 0,
                    len: 2,
                    packet: vec![0x10, 0x15],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(&gsmtap_msg.payload, &[0x10, 0x15,]);
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1811);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 3);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v24_binary = &[
        0x10, 0x00, 0x2c, 0x00, 0x2c, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x18, 0x0f, 0x22, 0x00, 0x68, 0x00, 0xe4, 0x0c, 0x00, 0x00, 0x09, 0xdc, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x0d, 0x00, 0x40, 0x85, 0x8e, 0xc4, 0xe5, 0xbf, 0xe0, 0x50, 0xdc, 0x29,
        0x15, 0x16, 0x00,
    ];
    let (_, parsed) = Message::from_bytes((v24_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 44,
            inner_length: 44,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 24,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 15,
                    rrc_rel_min: 34,
                    bearer_id: 0,
                    phy_cell_id: 104,
                    earfcn: 3300,
                    sfn_subfn: 56329,
                    pdu_num: 5,
                    sib_mask: 0,
                    len: 13,
                    packet: vec![
                        0x40, 0x85, 0x8e, 0xc4, 0xe5, 0xbf, 0xe0, 0x50, 0xdc, 0x29, 0x15, 0x16, 0x0
                    ],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[
            0x40, 0x85, 0x8e, 0xc4, 0xe5, 0xbf, 0xe0, 0x50, 0xdc, 0x29, 0x15, 0x16, 0x00,
        ]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 3300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 3520);
    assert_eq!(gsmtap_msg.header.subtype, 6);
    assert_eq!(gsmtap_msg.header.subslot, 9);

    let v20_binary = &[
        0x10, 0x00, 0x37, 0x00, 0x37, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x14, 0x0e, 0x30, 0x01, 0x09, 0x01, 0x9c, 0x18, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00,
        0x00, 0x00, 0x00, 0x18, 0x00, 0x08, 0x10, 0xa7, 0x14, 0x53, 0x59, 0xa6, 0x05, 0x43, 0x68,
        0xc0, 0x3b, 0xda, 0x30, 0x04, 0xa6, 0x88, 0x02, 0x8d, 0xa2, 0x00, 0x9a, 0x68, 0x40,
    ];
    let (_, parsed) = Message::from_bytes((v20_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 55,
            inner_length: 55,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 20,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 14,
                    rrc_rel_min: 48,
                    bearer_id: 1,
                    phy_cell_id: 265,
                    earfcn: 6300,
                    sfn_subfn: 0,
                    pdu_num: 9,
                    sib_mask: 0,
                    len: 24,
                    packet: vec![
                        0x8, 0x10, 0xa7, 0x14, 0x53, 0x59, 0xa6, 0x5, 0x43, 0x68, 0xc0, 0x3b, 0xda,
                        0x30, 0x4, 0xa6, 0x88, 0x2, 0x8d, 0xa2, 0x0, 0x9a, 0x68, 0x40
                    ],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[
            0x08, 0x10, 0xa7, 0x14, 0x53, 0x59, 0xa6, 0x05, 0x43, 0x68, 0xc0, 0x3b, 0xda, 0x30,
            0x04, 0xa6, 0x88, 0x02, 0x8d, 0xa2, 0x00, 0x9a, 0x68, 0x40,
        ]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 6300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 3);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v19_binary = &[
        0x10, 0x00, 0x28, 0x00, 0x28, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x13, 0x0e, 0x22, 0x00, 0x0b, 0x00, 0xfa, 0x09, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00,
        0x00, 0x00, 0x00, 0x09, 0x00, 0x28, 0x18, 0x40, 0x16, 0x08, 0x08, 0x80, 0x00, 0x00,
    ];
    let (_, parsed) = Message::from_bytes((v19_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 40,
            inner_length: 40,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 19,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 14,
                    rrc_rel_min: 34,
                    bearer_id: 0,
                    phy_cell_id: 11,
                    earfcn: 2554,
                    sfn_subfn: 0,
                    pdu_num: 50,
                    sib_mask: 0,
                    len: 9,
                    packet: vec![0x28, 0x18, 0x40, 0x16, 0x8, 0x8, 0x80, 0x0, 0x0],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[0x28, 0x18, 0x40, 0x16, 0x08, 0x08, 0x80, 0x00, 0x00,]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 2554);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 16);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v15_binary = &[
        0x10, 0x00, 0x26, 0x00, 0x26, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x0f, 0x0d, 0x21, 0x00, 0x9e, 0x00, 0x14, 0x05, 0x00, 0x00, 0x49, 0x8c, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x07, 0x00, 0x40, 0x0c, 0x8e, 0xc9, 0x42, 0x89, 0xe0,
    ];
    let (_, parsed) = Message::from_bytes((v15_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 38,
            inner_length: 38,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 15,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 13,
                    rrc_rel_min: 33,
                    bearer_id: 0,
                    phy_cell_id: 158,
                    earfcn: 1300,
                    sfn_subfn: 35913,
                    pdu_num: 5,
                    sib_mask: 0,
                    len: 7,
                    packet: vec![0x40, 0xc, 0x8e, 0xc9, 0x42, 0x89, 0xe0],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[0x40, 0x0c, 0x8e, 0xc9, 0x42, 0x89, 0xe0,]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 2244);
    assert_eq!(gsmtap_msg.header.subtype, 6);
    assert_eq!(gsmtap_msg.header.subslot, 9);

    let v15_binary = &[
        0x10, 0x00, 0x3b, 0x00, 0x3b, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x0f, 0x0d, 0x21, 0x01, 0x9e, 0x00, 0x14, 0x05, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00,
        0x00, 0x00, 0x00, 0x1c, 0x00, 0x08, 0x10, 0xa5, 0x34, 0x61, 0x41, 0xa3, 0x1c, 0x31, 0x68,
        0x04, 0x40, 0x1a, 0x00, 0x49, 0x16, 0x7c, 0x23, 0x15, 0x9f, 0x00, 0x10, 0x67, 0xc1, 0x06,
        0xd9, 0xe0, 0x00,
    ];
    let (_, parsed) = Message::from_bytes((v15_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 59,
            inner_length: 59,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 15,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 13,
                    rrc_rel_min: 33,
                    bearer_id: 1,
                    phy_cell_id: 158,
                    earfcn: 1300,
                    sfn_subfn: 0,
                    pdu_num: 9,
                    sib_mask: 0,
                    len: 28,
                    packet: vec![
                        0x8, 0x10, 0xa5, 0x34, 0x61, 0x41, 0xa3, 0x1c, 0x31, 0x68, 0x4, 0x40, 0x1a,
                        0x0, 0x49, 0x16, 0x7c, 0x23, 0x15, 0x9f, 0x0, 0x10, 0x67, 0xc1, 0x6, 0xd9,
                        0xe0, 0x0
                    ],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[
            0x08, 0x10, 0xa5, 0x34, 0x61, 0x41, 0xa3, 0x1c, 0x31, 0x68, 0x04, 0x40, 0x1a, 0x00,
            0x49, 0x16, 0x7c, 0x23, 0x15, 0x9f, 0x00, 0x10, 0x67, 0xc1, 0x06, 0xd9, 0xe0, 0x00,
        ]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 3);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v13_binary = &[
        0x10, 0x00, 0x21, 0x00, 0x21, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x0d, 0x0c, 0x74, 0x01, 0x32, 0x00, 0x38, 0x18, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x2c, 0x00,
    ];
    let (_, parsed) = Message::from_bytes((v13_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 33,
            inner_length: 33,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 13,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 12,
                    rrc_rel_min: 116,
                    bearer_id: 1,
                    phy_cell_id: 50,
                    earfcn: 6200,
                    sfn_subfn: 0,
                    pdu_num: 8,
                    sib_mask: 0,
                    len: 2,
                    packet: vec![0x2c, 0x0],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(&gsmtap_msg.payload, &[0x2c, 0x00]);
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 6200);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 0);
    assert_eq!(gsmtap_msg.header.subtype, 3);
    assert_eq!(gsmtap_msg.header.subslot, 0);

    let v9_binary = &[
        0x10, 0x00, 0x26, 0x00, 0x26, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x09, 0x0b, 0x70, 0x00, 0x00, 0x01, 0x14, 0x05, 0x00, 0x00, 0x09, 0x91, 0x0b, 0x00,
        0x00, 0x00, 0x00, 0x07, 0x00, 0x40, 0x0b, 0x8e, 0xc1, 0xdd, 0x13, 0xb0,
    ];
    let (_, parsed) = Message::from_bytes((v9_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 38,
            inner_length: 38,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 9,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 11,
                    rrc_rel_min: 112,
                    bearer_id: 0,
                    phy_cell_id: 256,
                    earfcn: 1300,
                    sfn_subfn: 37129,
                    pdu_num: 11,
                    sib_mask: 0,
                    len: 7,
                    packet: vec![0x40, 0xb, 0x8e, 0xc1, 0xdd, 0x13, 0xb0],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[0x40, 0x0b, 0x8e, 0xc1, 0xdd, 0x13, 0xb0,]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 2320);
    assert_eq!(gsmtap_msg.header.subtype, 6);
    assert_eq!(gsmtap_msg.header.subslot, 9);

    let v8_binary = &[
        0x10, 0x00, 0x21, 0x00, 0x21, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x08, 0x0a, 0x72, 0x01, 0x0e, 0x00, 0x9c, 0x18, 0x00, 0x00, 0xa9, 0x33, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x2e, 0x02,
    ];
    let (_, parsed) = Message::from_bytes((v8_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 33,
            inner_length: 33,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 8,
                packet: LteRrcOtaPacket::V8 {
                    rrc_rel_maj: 10,
                    rrc_rel_min: 114,
                    bearer_id: 1,
                    phy_cell_id: 14,
                    earfcn: 6300,
                    sfn_subfn: 13225,
                    pdu_num: 6,
                    sib_mask: 0,
                    len: 2,
                    packet: vec![0x2e, 0x2],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(&gsmtap_msg.payload, &[0x2e, 0x02]);
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 6300);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 826);
    assert_eq!(gsmtap_msg.header.subtype, 1);
    assert_eq!(gsmtap_msg.header.subslot, 9);

    let v6_binary = &[
        0x10, 0x00, 0x2f, 0x00, 0x2f, 0x00, 0xc0, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x06, 0x09, 0xb1, 0x00, 0x07, 0x01, 0x2c, 0x07, 0x25, 0x34, 0x02, 0x02, 0x00, 0x00,
        0x00, 0x12, 0x00, 0x40, 0x49, 0x88, 0x05, 0xc0, 0x97, 0x02, 0xd3, 0xb0, 0x98, 0x1c, 0x20,
        0xa0, 0x81, 0x8c, 0x43, 0x26, 0xd0,
    ];
    let (_, parsed) = Message::from_bytes((v6_binary, 0)).unwrap();
    assert_eq!(
        &parsed,
        &Message::Log {
            pending_msgs: 0,
            outer_length: 47,
            inner_length: 47,
            timestamp: Timestamp { ts: 0 },
            log_type: 45248,
            body: LogBody::LteRrcOtaMessage {
                ext_header_version: 6,
                packet: LteRrcOtaPacket::V5 {
                    rrc_rel_maj: 9,
                    rrc_rel_min: 177,
                    bearer_id: 0,
                    phy_cell_id: 263,
                    earfcn: 1836,
                    sfn_subfn: 13349,
                    pdu_num: 2,
                    sib_mask: 2,
                    len: 18,
                    packet: vec![
                        0x40, 0x49, 0x88, 0x5, 0xc0, 0x97, 0x2, 0xd3, 0xb0, 0x98, 0x1c, 0x20, 0xa0,
                        0x81, 0x8c, 0x43, 0x26, 0xd0
                    ],
                },
            },
        }
    );
    let (_, gsmtap_msg) = gsmtap_parser::parse(parsed).unwrap().unwrap();
    assert_eq!(
        &gsmtap_msg.payload,
        &[
            0x40, 0x49, 0x88, 0x05, 0xc0, 0x97, 0x02, 0xd3, 0xb0, 0x98, 0x1c, 0x20, 0xa0, 0x81,
            0x8c, 0x43, 0x26, 0xd0,
        ]
    );
    assert_eq!(gsmtap_msg.header.packet_type, 13);
    assert_eq!(gsmtap_msg.header.timeslot, 0);
    assert_eq!(gsmtap_msg.header.arfcn, 1836);
    assert_eq!(gsmtap_msg.header.signal_dbm, 0);
    assert_eq!(gsmtap_msg.header.signal_noise_ratio_db, 0);
    assert_eq!(gsmtap_msg.header.frame_number, 834);
    assert_eq!(gsmtap_msg.header.subtype, 5);
    assert_eq!(gsmtap_msg.header.subslot, 5);
}
