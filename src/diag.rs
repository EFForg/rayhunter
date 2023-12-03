use crate::hdlc::{hdlc_encapsulate, hdlc_decapsulate};

use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use bytes::{Buf, BufMut};
use std::os::fd::AsRawFd;
use std::thread;
use thiserror::Error;
use crc::{Crc, Algorithm};
use deku::prelude::*;

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const USER_SPACE_DATA_TYPE: i32 = 32;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

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

pub type DiagResult<T> = Result<T, DiagDeviceError>;

#[derive(Error, Debug)]
pub enum DiagDeviceError {
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(String),
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

// kinda unpleasant deku hackery here. deku expects an enum's variant to be
// right before its data, but in this case, a status value comes between the
// variants and the data. so we need to use deku's context (ctx) feature to pass
// those opcodes down to their respective parsers.
#[derive(Debug, Clone, DekuRead)]
pub struct Response {
    opcode: u32,
    subopcode: u32,
    status: u32,
    #[deku(ctx = "*opcode, *subopcode")]
    payload: ResponsePayload,
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

    pub fn try_clone(&self) -> DiagResult<Self> {
        Ok(DiagDevice {
            file: self.file.try_clone()?,
            crc: Crc::<u16>::new(&CRC_CCITT_ALG),
            use_mdm: self.use_mdm,
        })
    }

    pub fn read_response(&mut self) -> DiagResult<Option<Vec<Response>>> {
        let mut buf = vec![0; BUFFER_LEN];
        let bytes_read = self.file.read(&mut buf)?;
        if bytes_read < 4 {
            let msg = format!("read {} bytes from diag device, expected > 4", bytes_read);
            return Err(DiagDeviceError::DeviceReadFailed(msg));
        }
        let mut reader = Cursor::new(buf);

        if reader.get_i32_le() != USER_SPACE_DATA_TYPE {
            return Ok(None);
        }

        let num_messages = reader.get_u32_le();
        let mut messages = Vec::new();

        for _ in 0..num_messages {
            let msg_len = reader.get_u32_le() as usize;
            let mut msg = vec![0; msg_len];
            reader.read_exact(&mut msg)?;
            match Response::from_bytes((&hdlc_decapsulate(msg, &self.crc), 0)) {
                // todo: handle leftover bytes
                Ok(((_, leftover_bytes), res)) => {
                    if leftover_bytes > 0 {
                        println!("warning: {} leftover bytes when parsing response", leftover_bytes);
                    }
                    messages.push(res);
                },
                Err(e) => {
                    println!("error parsing response: {:?}", e);
                    continue;
                }
            }
        }

        Ok(Some(messages))
    }

    pub fn write_request(&mut self, req: Request) -> DiagResult<()> {
        let mut buf: Vec<u8> = vec![];
        buf.put_i32_le(USER_SPACE_DATA_TYPE);
        if self.use_mdm > 0 {
            buf.put_i32_le(-1);
        }
        buf.extend(hdlc_encapsulate(req.to_bytes().unwrap(), &self.crc));
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

    pub fn config_logs(&mut self) -> DiagResult<()> {
        // todo: replace panics w/ errors

        println!("retrieving diag logging capabilities...");
        self.write_request(Request::LogConfig(LogConfigRequest::RetrieveIdRanges))?;

        let res = self.read_response()?
            .expect("got unexpected non-userspace message from device")
            .pop().expect("no LogConfigRequest::RetrieveIdRanges response received");
        if res.status != 0 {
            let msg = format!("LogConfigRequest::RetrieveIdRanges failed with status {}", res.status);
            return Err(DiagDeviceError::DeviceReadFailed(msg));
        }
        if let ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges { log_mask_sizes }) = res.payload {
            // for each log type, send a logging mask of all 1's equal to its respective mask size
            for (log_type, &log_mask_bitsize) in log_mask_sizes.iter().enumerate() {
                if log_mask_bitsize == 0 {
                    continue;
                }
                self.write_request(build_log_mask_request(log_type as u32, log_mask_bitsize))?;
                let set_mask_res = self.read_response()?
                    .expect("unexpected non-userspace message from device")
                    .pop().expect("expected response, got none");
                if set_mask_res.status != 0 {
                    eprintln!("LogConfigRequest::SetMask failed with status {}", set_mask_res.status);
                }
                if let ResponsePayload::LogConfig(LogConfigResponse::SetMask) = set_mask_res.payload {
                    println!("registered logging for type {}", log_type);
                } else {
                    panic!("unexpected response payload: {:?}", set_mask_res.payload);
                }
            }
        } else {
            panic!("unexpected response payload: {:?}", res.payload);
        }

        Ok(())
    }
}

// register logging for each supported log type. it seems that "log_mask_sizes" is an array of
// numbers for each log type, where each number is how many bits are in that log mask
fn build_log_mask_request(log_type: u32, log_mask_bitsize: u32) -> Request {
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
}
