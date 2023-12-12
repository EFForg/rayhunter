use crate::hdlc::{hdlc_encapsulate, hdlc_decapsulate, HdlcError};
use crate::diag::{Message, ResponsePayload, Request, LogConfigRequest, LogConfigResponse, build_log_mask_request, RequestContainer, DataType, MessagesContainer};
use crate::log_codes;

use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use thiserror::Error;
use crc::{Crc, Algorithm};
use deku::prelude::*;

pub type DiagResult<T> = Result<T, DiagDeviceError>;

#[derive(Error, Debug)]
pub enum DiagDeviceError {
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(String),
    #[error("Nonzero status code {0} for diag request: {1:?}")]
    RequestFailed(u32, Request),
    #[error("Didn't receive response for request: {0:?}")]
    NoResponse(Request),
    #[error("HDLC error {0}")]
    HdlcError(#[from] HdlcError),
    #[error("Deku error {0}")]
    DekuError(#[from] DekuError),
}

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

pub const LOG_CODES_FOR_RAW_PACKET_LOGGING: [u32; 11] = [
    // Layer 2:
    log_codes::LOG_GPRS_MAC_SIGNALLING_MESSAGE_C, // 0x5226

    // Layer 3:
    log_codes::LOG_GSM_RR_SIGNALING_MESSAGE_C, // 0x512f
    log_codes::WCDMA_SIGNALLING_MESSAGE, // 0x412f
    log_codes::LOG_LTE_RRC_OTA_MSG_LOG_C, // 0xb0c0
    log_codes::LOG_NR_RRC_OTA_MSG_LOG_C, // 0xb821
    
    // NAS:
    log_codes::LOG_UMTS_NAS_OTA_MESSAGE_LOG_PACKET_C, // 0x713a
    log_codes::LOG_LTE_NAS_ESM_OTA_IN_MSG_LOG_C, // 0xb0e2
    log_codes::LOG_LTE_NAS_ESM_OTA_OUT_MSG_LOG_C, // 0xb0e3
    log_codes::LOG_LTE_NAS_EMM_OTA_IN_MSG_LOG_C, // 0xb0ec
    log_codes::LOG_LTE_NAS_EMM_OTA_OUT_MSG_LOG_C, // 0xb0ed
    
    // User IP traffic:
    log_codes::LOG_DATA_PROTOCOL_LOGGING_C // 0x11eb
];

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

pub struct DiagDevice<'a> {
    file: &'a File,
    accumulator: Vec<u8>,
    use_mdm: i32,
    crc: Crc<u16>,
}

impl<'a> DiagDevice<'a> {
    pub fn new(file: &'a File) -> DiagResult<Self> {
        let fd = file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            accumulator: vec![],
            file,
            crc: Crc::<u16>::new(&CRC_CCITT_ALG),
            use_mdm,
        })
    }

    fn parse_response_container(&self, container: MessagesContainer) -> DiagResult<Vec<Message>> {
        let mut result = Vec::new();
        for msg in container.messages {
            match hdlc_decapsulate(&msg.data, &self.crc) {
                Ok(data) => match Message::from_bytes((&data, 0)) {
                    Ok(((leftover_bytes, _), res)) => {
                        if leftover_bytes.len() > 0 {
                            println!("warning: {} leftover bytes when parsing Message", leftover_bytes.len());
                        }
                        result.push(res);
                    },
                    Err(e) => {
                        println!("error parsing response: {:?}", e);
                        println!("{:?}", data);
                    },
                },
                Err(err) => {
                    println!("error decapsulating response: {:?}", err);
                    println!("{:?}", &msg.data);
                }
            }
        }
        Ok(result)
    }

    pub fn read_response(&mut self) -> DiagResult<Vec<Message>> {
        let mut read_buf = vec![0; BUFFER_LEN];
        loop {
            let bytes_read = self.file.read(&mut read_buf).unwrap();
            let ((leftover_bytes, _), res_container) = MessagesContainer::from_bytes((&read_buf[0..bytes_read], 0))?;
            if leftover_bytes.len() > 0 {
                println!("warning: {} leftover bytes when parsing MessagesContainer", leftover_bytes.len());
            }
            if res_container.data_type == DataType::UserSpace {
                return self.parse_response_container(res_container);
            } else {
                println!("skipping non-userspace message...")
            }
        }
    }

    pub fn write_request(&mut self, req: &Request) -> DiagResult<()> {
        let buf = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: self.use_mdm > 0,
            mdm_field: -1,
            hdlc_encapsulated_request: hdlc_encapsulate(&req.to_bytes().unwrap(), &self.crc),
        }.to_bytes().unwrap();
        unsafe {
            let fd = self.file.as_raw_fd();
            let buf_ptr = buf.as_ptr() as *const libc::c_void;
            let ret = libc::write(fd, buf_ptr, buf.len());
            if ret < 0 {
                let msg = format!("write failed with error code {}", ret);
                return Err(DiagDeviceError::DeviceReadFailed(msg));
            }
        }
        Ok(())
    }

    fn retrieve_id_ranges(&mut self) -> DiagResult<[u32; 16]> {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        self.write_request(&req)?;

        for msg in self.read_response()? {
            match msg {
                Message::Log { .. } => println!("skipping log response..."),
                Message::Response { payload, status, .. } => match payload {
                    ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges { log_mask_sizes }) => {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(log_mask_sizes);
                    },
                    _ => println!("skipping non-LogConfigResponse response..."),
                },
            }
        }

        Err(DiagDeviceError::NoResponse(req))
    }

    fn set_log_mask(&mut self, log_type: u32, log_mask_bitsize: u32) -> DiagResult<()> {
        let req = build_log_mask_request(log_type, log_mask_bitsize, &LOG_CODES_FOR_RAW_PACKET_LOGGING);
        self.write_request(&req)?;

        for msg in self.read_response()? {
            match msg {
                Message::Log { .. } => println!("skipping log response..."),
                Message::Response { payload, status, .. } => {
                    if let ResponsePayload::LogConfig(LogConfigResponse::SetMask) = payload {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(());
                    }
                },
            }
        }

        Err(DiagDeviceError::NoResponse(req))
    }

    pub fn config_logs(&mut self) -> DiagResult<()> {
        println!("retrieving diag logging capabilities...");
        let log_mask_sizes = self.retrieve_id_ranges()?;

        for (log_type, &log_mask_bitsize) in log_mask_sizes.iter().enumerate() {
            if log_mask_bitsize > 0 {
                self.set_log_mask(log_type as u32, log_mask_bitsize)?;
                println!("enabled logging for log type {}", log_type);
            }
        }

        Ok(())
    }
}

// Triggers the diag device's debug logging mode
fn enable_frame_readwrite(fd: i32, mode: i32) -> DiagResult<()> {
    unsafe {
        if libc::ioctl(fd, DIAG_IOCTL_SWITCH_LOGGING.into(), mode, 0, 0, 0) < 0 {
            let ret = libc::ioctl(
                fd,
                DIAG_IOCTL_SWITCH_LOGGING.into(),
                &mut [mode, -1, 0] as *mut _, // diag_logging_mode_param_t
                std::mem::size_of::<[i32; 3]>(), 0, 0, 0, 0
            );
            if ret < 0 {
                let msg = format!("DIAG_IOCTL_SWITCH_LOGGING ioctl failed with error code {}", ret);
                return Err(DiagDeviceError::InitializationFailed(msg))
            }
        }
    }
    Ok(())
}

// Unsure of what MDM actually stands for, but if `use_mdm` is > 0, then
// an additional mask is included in every diag request
fn determine_use_mdm(fd: i32) -> DiagResult<i32> {
    let use_mdm: i32 = 0;
    unsafe {
        if libc::ioctl(fd, DIAG_IOCTL_REMOTE_DEV.into(), &use_mdm as *const i32) < 0 {
            let msg = format!("DIAG_IOCTL_REMOTE_DEV ioctl failed with error code {}", 0);
            return Err(DiagDeviceError::InitializationFailed(msg))
        }
    }
    Ok(use_mdm)
}
