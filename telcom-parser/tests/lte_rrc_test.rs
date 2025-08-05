use asn1_codecs::{PerCodecData, uper::UperCodec};
use telcom_parser::lte_rrc::BCCH_DL_SCH_Message;

fn hex_to_bin(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test() {
    let data = hex_to_bin("484c469010600018fd1a9207e22103108ac21bdc09802292cdd20000");
    let mut asn_data = PerCodecData::from_slice_uper(&data);
    let sib1 = BCCH_DL_SCH_Message::uper_decode(&mut asn_data);
    dbg!(&sib1);
}
