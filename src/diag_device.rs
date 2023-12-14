use crate::hdlc::{hdlc_encapsulate, HdlcError};
use crate::diag::{Message, ResponsePayload, Request, LogConfigRequest, LogConfigResponse, build_log_mask_request, RequestContainer, DataType, MessagesContainer};
use crate::diag_reader::{DiagReader, CRC_CCITT};
use crate::debug_file::DebugFileBlock;
use crate::log_codes;

use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use thiserror::Error;
use log::{info, warn, error};
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

pub struct DiagDevice {
    file: File,
    debug_file: Option<File>,
    read_buf: Vec<u8>,
    use_mdm: i32,
}

impl DiagReader for DiagDevice {
    fn get_next_messages_container(&mut self) -> DiagResult<MessagesContainer> {
        let bytes_read = self.file.read(&mut self.read_buf)?;
        if let Some(debug_file) = self.debug_file.as_mut() {
            let debug_block = DebugFileBlock {
                size: bytes_read as u32,
                data: &self.read_buf[0..bytes_read],
            };
            let debug_block_bytes = debug_block.to_bytes()?;
            debug_file.write_all(&debug_block_bytes)?;
        }
        let ((leftover_bytes, _), container) = MessagesContainer::from_bytes((&self.read_buf[0..bytes_read], 0))?;
        if leftover_bytes.len() > 0 {
            warn!("warning: {} leftover bytes when parsing MessagesContainer", leftover_bytes.len());
        }
        Ok(container)
    }
}

impl DiagDevice {
    pub fn new() -> DiagResult<Self> {
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .open("/dev/diag")?;
        let fd = file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            read_buf: vec![0; BUFFER_LEN],
            file,
            debug_file: None,
            use_mdm,
        })
    }

    // Creates a file at the given path where all binary output from /dev/diag
    // will be recorded.
    pub fn enable_debug_mode<P>(&mut self, path: P) -> DiagResult<()> where P: AsRef<std::path::Path> {
        let debug_file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(path)?;
        info!("enabling debug mode, writing debug output to {:?}", debug_file);
        self.debug_file = Some(debug_file);
        Ok(())
    }

    pub fn write_request(&mut self, req: &Request) -> DiagResult<()> {
        let buf = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: self.use_mdm > 0,
            mdm_field: -1,
            hdlc_encapsulated_request: hdlc_encapsulate(&req.to_bytes()?, &CRC_CCITT),
        }.to_bytes()?;
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
                Message::Log { .. } => info!("skipping log response..."),
                Message::Response { payload, status, .. } => match payload {
                    ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges { log_mask_sizes }) => {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(log_mask_sizes);
                    },
                    _ => info!("skipping non-LogConfigResponse response..."),
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
                Message::Log { .. } => info!("skipping log response..."),
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
        info!("retrieving diag logging capabilities...");
        let log_mask_sizes = self.retrieve_id_ranges()?;

        for (log_type, &log_mask_bitsize) in log_mask_sizes.iter().enumerate() {
            if log_mask_bitsize > 0 {
                self.set_log_mask(log_type as u32, log_mask_bitsize)?;
                info!("enabled logging for log type {}", log_type);
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
