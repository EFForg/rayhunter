//! Diag protocol serialization/deserialization

use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite)]
pub struct RequestContainer {
    pub data_type: DataType,
    #[deku(skip)]
    pub use_mdm: bool,
    #[deku(skip, cond = "!*use_mdm")]
    pub mdm_field: i32,
    pub hdlc_encapsulated_request: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(type = "u32")]
pub enum Request {
    #[deku(id = "115")]
    LogConfig(LogConfigRequest),
}

#[derive(Debug, Clone, PartialEq, DekuWrite)]
#[deku(type = "u32", endian = "little")]
pub enum LogConfigRequest {
    #[deku(id = "1")]
    RetrieveIdRanges,

    #[deku(id = "3")]
    SetMask {
        log_type: u32,
        log_mask_bitsize: u32,
        log_mask: Vec<u8>,
    }
}

#[derive(Debug, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u32", endian = "little")]
pub enum DataType {
    #[deku(id = "32")]
    UserSpace,
    #[deku(id_pat = "_")]
    Other(u32),
}

#[derive(Debug, Clone, DekuRead)]
pub struct ResponseContainer {
    pub data_type: DataType,
    pub num_responses: u32,
    #[deku(count = "num_responses")]
    pub responses: Vec<HdlcEncapsulatedResponse>,
}

#[derive(Debug, Clone, DekuRead)]
pub struct HdlcEncapsulatedResponse {
    pub len: u32,
    #[deku(count = "len")]
    pub data: Vec<u8>,
}

// kinda unpleasant deku hackery here. deku expects an enum's variant to be
// right before its data, but in this case, a status value comes between the
// variants and the data. so we need to use deku's context (ctx) feature to pass
// those opcodes down to their respective parsers.
#[derive(Debug, Clone, DekuRead)]
pub struct Response {
    opcode: u32,
    subopcode: u32,
    pub status: u32,
    #[deku(ctx = "*opcode, *subopcode")]
    pub payload: ResponsePayload,
}

#[derive(Debug, Clone, DekuRead)]
#[deku(ctx = "opcode: u32, subopcode: u32", id = "opcode")]
pub enum ResponsePayload {
    #[deku(id = "115")]
    LogConfig(#[deku(ctx = "subopcode")] LogConfigResponse),
}

#[derive(Debug, Clone, DekuRead)]
#[deku(ctx = "subopcode: u32", id = "subopcode")]
pub enum LogConfigResponse {
    #[deku(id = "1")]
    RetrieveIdRanges {
        log_mask_sizes: [u32; 16],
    },

    #[deku(id = "3")]
    SetMask,
}

// register logging for each supported log type. it seems that "log_mask_sizes" is an array of
// numbers for each log type, where each number is how many bits are in that log mask
pub fn build_log_mask_request(log_type: u32, log_mask_bitsize: u32) -> Request {
    // if log_mask_bitsize = 8n + k, then we need n+1 bytes to store the mask, with the last
    // byte having k bits set
    let mask_len = (log_mask_bitsize as usize + 7) / 8;
    let mut log_mask = vec![0xff; mask_len];
    if log_mask_bitsize % 8 != 0 {
        log_mask[mask_len - 1] = 0xff >> (8 - (log_mask_bitsize as usize % 8));
    }

    Request::LogConfig(LogConfigRequest::SetMask {
        log_type: log_type as u32,
        log_mask_bitsize,
        log_mask,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        assert_eq!(req.to_bytes().unwrap(), vec![115, 0, 0, 0, 1, 0, 0, 0]);

        let req = Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 0,
            log_mask: vec![],
        });
        assert_eq!(req.to_bytes().unwrap(), vec![
            115, 0, 0, 0,
            3, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
        ]);
    }

    #[test]
    fn test_build_log_mask_request() {
        assert_eq!(build_log_mask_request(0, 1), Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 1,
            log_mask: vec![0x01],
        }));
        assert_eq!(build_log_mask_request(0, 2), Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 2,
            log_mask: vec![0x03],
        }));
        assert_eq!(build_log_mask_request(0, 8), Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 8,
            log_mask: vec![0xff],
        }));
        assert_eq!(build_log_mask_request(0, 9), Request::LogConfig(LogConfigRequest::SetMask {
            log_type: 0,
            log_mask_bitsize: 9,
            log_mask: vec![0xff, 0x01],
        }));
    }

    #[test]
    fn test_request_container() {
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: false,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(req.to_bytes().unwrap(), vec![
            32, 0, 0, 0,
            1, 2, 3, 4,
        ]);
        let req = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: true,
            mdm_field: -1,
            hdlc_encapsulated_request: vec![1, 2, 3, 4],
        };
        assert_eq!(req.to_bytes().unwrap(), vec![
            32, 0, 0, 0,
            255, 255, 255, 255,
            1, 2, 3, 4,
        ]);
    }
}
