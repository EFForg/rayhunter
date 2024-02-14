use crate::hdlc::hdlc_encapsulate;
use crate::diag::{Message, ResponsePayload, Request, LogConfigRequest, LogConfigResponse, build_log_mask_request, RequestContainer, DataType, MessagesContainer};
use crate::diag_reader::{DiagReader, CRC_CCITT};
use crate::qmdl::QmdlWriter;
use crate::log_codes;

use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use thiserror::Error;
use log::{info, warn, error};
use deku::prelude::*;

pub type DiagResult<T> = Result<T, DiagDeviceError>;

#[derive(Error, Debug)]
pub enum DiagDeviceError {
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(std::io::Error),
    #[error("Failed to write diag device: {0}")]
    DeviceWriteFailed(String),
    #[error("Nonzero status code {0} for diag request: {1:?}")]
    RequestFailed(u32, Request),
    #[error("Didn't receive response for request: {0:?}")]
    NoResponse(Request),
    #[error("Failed to open QMDL file: {0}")]
    OpenQmdlFileError(std::io::Error),
    #[error("Failed to write to QMDL file: {0}")]
    QmdlFileWriteError(std::io::Error),
    #[error("Failed to open diag device: {0}")]
    OpenDiagDeviceError(std::io::Error),
    #[error("Failed to parse MessagesContainer: {0}")]
    ParseMessagesContainerError(deku::DekuError),
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

#[cfg(target_arch = "arm")]
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
#[cfg(target_arch = "x86_64")]
const DIAG_IOCTL_REMOTE_DEV: u64 = 32;

#[cfg(target_arch = "arm")]
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;
#[cfg(target_arch = "x86_64")]
const DIAG_IOCTL_SWITCH_LOGGING: u64 = 7;

pub struct DiagDevice {
    file: File,
    pub qmdl_writer: Option<QmdlWriter<File>>,
    fully_initialized: bool,
    read_buf: Vec<u8>,
    use_mdm: i32,
}

impl DiagReader for DiagDevice {
    type Err = DiagDeviceError;

    fn get_next_messages_container(&mut self) -> DiagResult<MessagesContainer> {
        let mut bytes_read = 0;
        while bytes_read == 0 {
            bytes_read = self.file.read(&mut self.read_buf)
                .map_err(DiagDeviceError::DeviceReadFailed)?;
        }
        let ((leftover_bytes, _), container) = MessagesContainer::from_bytes((&self.read_buf[0..bytes_read], 0))
            .map_err(DiagDeviceError::ParseMessagesContainerError)?;
        if !leftover_bytes.is_empty() {
            warn!("warning: {} leftover bytes when parsing MessagesContainer", leftover_bytes.len());
        }

        if let Some(qmdl_writer) = self.qmdl_writer.as_mut() {
            if self.fully_initialized {
                qmdl_writer.write_container(&container)
                    .map_err(DiagDeviceError::QmdlFileWriteError)?;
            }
        }
        Ok(container)
    }
}

impl DiagDevice {
    pub fn new(qmdl_writer: Option<QmdlWriter<File>>) -> DiagResult<Self> {
        let diag_file = std::fs::File::options()
            .read(true)
            .write(true)
            .open("/dev/diag")
            .map_err(DiagDeviceError::OpenDiagDeviceError)?;
        let fd = diag_file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            read_buf: vec![0; BUFFER_LEN],
            file: diag_file,
            fully_initialized: false,
            qmdl_writer,
            use_mdm,
        })
    }

    fn write_request(&mut self, req: &Request) -> DiagResult<()> {
        let req_bytes = &req.to_bytes().expect("Failed to serialize Request");
        let buf = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: self.use_mdm > 0,
            mdm_field: -1,
            hdlc_encapsulated_request: hdlc_encapsulate(req_bytes, &CRC_CCITT),
        }.to_bytes().expect("Failed to serialize RequestContainer");
        unsafe {
            let fd = self.file.as_raw_fd();
            let buf_ptr = buf.as_ptr() as *const libc::c_void;
            let ret = libc::write(fd, buf_ptr, buf.len());
            if ret < 0 {
                let msg = format!("write failed with error code {}", ret);
                return Err(DiagDeviceError::DeviceWriteFailed(msg));
            }
        }
        Ok(())
    }

    fn retrieve_id_ranges(&mut self) -> DiagResult<[u32; 16]> {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        self.write_request(&req)?;

        for msg in self.read_response()? {
            match msg {
                Ok(Message::Log { .. }) => info!("skipping log response..."),
                Ok(Message::Response { payload, status, .. }) => match payload {
                    ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges { log_mask_sizes }) => {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(log_mask_sizes);
                    },
                    _ => info!("skipping non-LogConfigResponse response..."),
                },
                Err(e) => error!("error parsing message: {:?}", e),
            }
        }

        Err(DiagDeviceError::NoResponse(req))
    }

    fn set_log_mask(&mut self, log_type: u32, log_mask_bitsize: u32) -> DiagResult<()> {
        let req = build_log_mask_request(log_type, log_mask_bitsize, &LOG_CODES_FOR_RAW_PACKET_LOGGING);
        self.write_request(&req)?;

        for msg in self.read_response()? {
            match msg {
                Ok(Message::Log { .. }) => info!("skipping log response..."),
                Ok(Message::Response { payload, status, .. }) => {
                    if let ResponsePayload::LogConfig(LogConfigResponse::SetMask) = payload {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(());
                    }
                },
                Err(e) => error!("error parsing message: {:?}", e),
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

        self.fully_initialized = true;
        Ok(())
    }
}

// Triggers the diag device's debug logging mode
fn enable_frame_readwrite(fd: i32, mode: i32) -> DiagResult<()> {
    unsafe {
        if libc::ioctl(fd, DIAG_IOCTL_SWITCH_LOGGING, mode, 0, 0, 0) < 0 {
            let ret = libc::ioctl(
                fd,
                DIAG_IOCTL_SWITCH_LOGGING,
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
        if libc::ioctl(fd, DIAG_IOCTL_REMOTE_DEV, &use_mdm as *const i32) < 0 {
            let msg = format!("DIAG_IOCTL_REMOTE_DEV ioctl failed with error code {}", 0);
            return Err(DiagDeviceError::InitializationFailed(msg))
        }
    }
    Ok(use_mdm)
}
