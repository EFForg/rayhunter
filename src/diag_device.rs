use crate::hdlc::{hdlc_encapsulate, hdlc_decapsulate, HdlcError};
use crate::diag::{Response, ResponsePayload, Request, LogConfigRequest, LogConfigResponse, build_log_mask_request, RequestContainer, DataType, ResponseContainer};

use std::fs::File;
use std::io::{Cursor, Read, Write};
use bytes::{Buf, BufMut};
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

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const USER_SPACE_DATA_TYPE: i32 = 32;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

pub struct DiagDevice {
    file: File,
    use_mdm: i32,
    crc: Crc<u16>,
}

impl DiagDevice {
    pub fn new() -> DiagResult<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .open("/dev/diag")?;
        let fd = file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            file,
            crc: Crc::<u16>::new(&CRC_CCITT_ALG),
            use_mdm,
        })
    }

    fn parse_response_container(&self, container: ResponseContainer) -> DiagResult<Vec<Response>> {
        let mut result = Vec::new();
        for msg in container.responses {
            let data = hdlc_decapsulate(msg.data, &self.crc)?;
            match Response::from_bytes((&data, 0)) {
                Ok(((_, leftover_bytes), res)) => {
                    if leftover_bytes > 0 {
                        println!("warning: {} leftover bytes when Response", leftover_bytes);
                    }
                    result.push(res);
                },
                Err(e) => {
                    println!("{:?}", data);
                    println!("error parsing response: {:?}", e);
                },
            }
        }
        Ok(result)
    }

    pub fn read_response(&mut self) -> DiagResult<Vec<Response>> {
        let mut buf = vec![0; BUFFER_LEN];

        loop {
            let _ = self.file.read(&mut buf)?;
            let ((_, leftover_bytes), res_container) = ResponseContainer::from_bytes((&buf, 0))?;
            if leftover_bytes > 0 {
                println!("warning: {} leftover bytes when parsing ResponseContainer", leftover_bytes);
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
            hdlc_encapsulated_request: hdlc_encapsulate(req.to_bytes().unwrap(), &self.crc),
        }.to_bytes().unwrap();
        unsafe {
            let fd = self.file.as_raw_fd();
            let buf_ptr = buf.as_ptr() as *const libc::c_void;
            let ret = libc::write(fd, buf_ptr, buf.len());
            if ret < 0 {
                let msg = format!("write failed with error code {}", ret);
                return Err(DiagDeviceError::DeviceReadFailed(msg));
            }
            println!("{}. wrote {} bytes to device", buf.len(), ret);
        }
        Ok(())
    }

    fn retrieve_id_ranges(&mut self) -> DiagResult<[u32; 16]> {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        self.write_request(&req)?;

        for res in self.read_response()? {
            match res.payload {
                ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges { log_mask_sizes }) => {
                    if res.status != 0 {
                        return Err(DiagDeviceError::RequestFailed(res.status, req));
                    }
                    return Ok(log_mask_sizes);
                },
                _ => println!("skipping non-LogConfigResponse response..."),
            }
        }

        return Err(DiagDeviceError::NoResponse(req));
    }

    fn set_log_mask(&mut self, log_type: u32, log_mask_bitsize: u32) -> DiagResult<()> {
        // send a logging mask of all 1's equal to its respective mask size
        let req = build_log_mask_request(log_type, log_mask_bitsize);
        self.write_request(&req)?;

        for res in self.read_response()? {
            if let ResponsePayload::LogConfig(LogConfigResponse::SetMask) = res.payload {
                if res.status != 0 {
                    return Err(DiagDeviceError::RequestFailed(res.status, req));
                }
                return Ok(());
            }
        }

        return Err(DiagDeviceError::NoResponse(req));
    }

    pub fn config_logs(&mut self) -> DiagResult<()> {
        println!("retrieving diag logging capabilities...");
        let log_mask_sizes = self.retrieve_id_ranges()?;
        println!("log mask sizes: {:?}", log_mask_sizes);

        for (log_type, &log_mask_bitsize) in log_mask_sizes.iter().enumerate() {
            if log_mask_bitsize > 0 {
                println!("setting logging for log_type {}", log_type);
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
