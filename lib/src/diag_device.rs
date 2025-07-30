use crate::diag::{
    CRC_CCITT, DataType, DiagParsingError, LogConfigRequest, LogConfigResponse, Message,
    MessagesContainer, Request, RequestContainer, ResponsePayload, build_log_mask_request,
};
use crate::hdlc::hdlc_encapsulate;
use crate::{Device, log_codes};

use deku::prelude::*;
use futures::TryStream;
use log::{debug, error, info};
use std::io::ErrorKind;
use std::os::fd::AsRawFd;
use std::time::Duration;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;

pub type DiagResult<T> = Result<T, DiagDeviceError>;

#[derive(Error, Debug)]
pub enum DiagDeviceError {
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(std::io::Error),
    #[error("Failed to write diag device: {0}")]
    DeviceWriteFailed(std::io::Error),
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

pub const LOG_CODES_FOR_RAW_PACKET_LOGGING: [u32; 13] = [
    // Layer 2:
    log_codes::LOG_GPRS_MAC_SIGNALLING_MESSAGE_C,
    // Layer 3:
    log_codes::LOG_GSM_RR_SIGNALING_MESSAGE_C,
    log_codes::WCDMA_SIGNALLING_MESSAGE,
    log_codes::LOG_LTE_RRC_OTA_MSG_LOG_C,
    log_codes::LOG_NR_RRC_OTA_MSG_LOG_C,
    // NAS:
    log_codes::LOG_UMTS_NAS_OTA_MESSAGE_LOG_PACKET_C,
    log_codes::LOG_LTE_NAS_ESM_OTA_IN_MSG_LOG_C,
    log_codes::LOG_LTE_NAS_ESM_OTA_OUT_MSG_LOG_C,
    log_codes::LOG_LTE_NAS_EMM_OTA_IN_MSG_LOG_C,
    log_codes::LOG_LTE_NAS_EMM_OTA_OUT_MSG_LOG_C,
    // MAC
    log_codes::LOG_LTE_MAC_DL,
    log_codes::LOG_LTE_MAC_UL,
    // User IP traffic:
    log_codes::LOG_DATA_PROTOCOL_LOGGING_C,
];

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const MEMORY_DEVICE_MODE: u32 = 2;

#[cfg(target_env = "musl")]
const DIAG_IOCTL_REMOTE_DEV: i32 = 32;
#[cfg(all(not(target_env = "musl"), target_arch = "arm"))]
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
#[cfg(all(not(target_env = "musl"), target_arch = "x86_64"))]
const DIAG_IOCTL_REMOTE_DEV: u64 = 32;
#[cfg(all(not(target_env = "musl"), target_arch = "aarch64"))]
const DIAG_IOCTL_REMOTE_DEV: u64 = 32;

#[cfg(target_env = "musl")]
const DIAG_IOCTL_SWITCH_LOGGING: i32 = 7;
#[cfg(all(not(target_env = "musl"), target_arch = "arm"))]
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;
#[cfg(all(not(target_env = "musl"), target_arch = "x86_64"))]
const DIAG_IOCTL_SWITCH_LOGGING: u64 = 7;
#[cfg(all(not(target_env = "musl"), target_arch = "aarch64"))]
const DIAG_IOCTL_SWITCH_LOGGING: u64 = 7;

pub struct DiagDevice {
    file: File,
    read_buf: Vec<u8>,
    use_mdm: i32,
}

impl DiagDevice {
    pub async fn new(configured_device: &Device) -> DiagResult<Self> {
        Self::new_with_retries(Duration::from_secs(30), configured_device).await
    }

    pub async fn new_with_retries(
        max_duration: Duration,
        configured_device: &Device,
    ) -> DiagResult<Self> {
        // For some reason the diag device needs a very long time to become available again with in
        // the same process, on TP-Link M7350 v3. While process restart would reset it faster.

        let start_time = std::time::Instant::now();
        let max_delay = Duration::from_secs(5);

        let mut delay = Duration::from_millis(100);
        let mut num_retries = 0;

        loop {
            match Self::try_new(configured_device).await {
                Ok(device) => {
                    info!("Diag device initialization succeeded after {num_retries} retries");
                    return Ok(device);
                }
                Err(e) => {
                    num_retries += 1;
                    if start_time.elapsed() >= max_duration {
                        error!("Failed to initialize diag device after {max_duration:?}: {e}");
                        return Err(e);
                    }

                    info!(
                        "Diag device initialization failed {num_retries} times, retrying in {delay:?}: {e}"
                    );
                    sleep(delay).await;

                    // Exponential backoff
                    delay = std::cmp::min(delay * 2, max_delay);
                }
            }
        }
    }

    async fn try_new(configured_device: &Device) -> DiagResult<Self> {
        let diag_file = File::options()
            .read(true)
            .write(true)
            .open("/dev/diag")
            .await
            .map_err(DiagDeviceError::OpenDiagDeviceError)?;
        let fd = diag_file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE, configured_device)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            read_buf: vec![0; BUFFER_LEN],
            file: diag_file,
            use_mdm,
        })
    }

    pub fn as_stream(
        &mut self,
    ) -> impl TryStream<Ok = MessagesContainer, Error = DiagDeviceError> + '_ {
        futures::stream::try_unfold(self, |dev| async {
            let container = dev.get_next_messages_container().await?;
            Ok(Some((container, dev)))
        })
    }

    async fn get_next_messages_container(&mut self) -> Result<MessagesContainer, DiagDeviceError> {
        let mut bytes_read = 0;
        // TP-Link M7350 sometimes sends too small messages, we need to be able to deal with short reads.
        while bytes_read <= 8 {
            bytes_read = self
                .file
                .read(&mut self.read_buf)
                .await
                .map_err(DiagDeviceError::DeviceReadFailed)?;
        }

        debug!(
            "Parsing messages container size = {:?} [{:?}]",
            bytes_read,
            &self.read_buf[0..bytes_read]
        );

        match MessagesContainer::from_bytes((&self.read_buf[0..bytes_read], 0)) {
            Ok((_, container)) => Ok(container),
            Err(err) => Err(DiagDeviceError::ParseMessagesContainerError(err)),
        }
    }

    async fn write_request(&mut self, req: &Request) -> DiagResult<()> {
        let req_bytes = &req.to_bytes().expect("Failed to serialize Request");
        let buf = RequestContainer {
            data_type: DataType::UserSpace,
            use_mdm: self.use_mdm > 0,
            mdm_field: -1,
            hdlc_encapsulated_request: hdlc_encapsulate(req_bytes, &CRC_CCITT),
        }
        .to_bytes()
        .expect("Failed to serialize RequestContainer");
        if let Err(err) = self.file.write(&buf).await {
            // For reasons I don't entirely understand, calls to write(2) on
            // /dev/diag always return 0 bytes written, though the written
            // requests end up being interpreted. As such, we're not concerned
            // about WriteZero errors
            if err.kind() != ErrorKind::WriteZero {
                return Err(DiagDeviceError::DeviceWriteFailed(err));
            }
        }
        if let Err(err) = self.file.flush().await
            && err.kind() != ErrorKind::WriteZero
        {
            return Err(DiagDeviceError::DeviceWriteFailed(err));
        }
        Ok(())
    }

    async fn read_response(&mut self) -> DiagResult<Vec<Result<Message, DiagParsingError>>> {
        loop {
            let container = self.get_next_messages_container().await?;
            if container.data_type != DataType::UserSpace {
                continue;
            }
            return Ok(container.into_messages());
        }
    }

    async fn retrieve_id_ranges(&mut self) -> DiagResult<[u32; 16]> {
        let req = Request::LogConfig(LogConfigRequest::RetrieveIdRanges);
        self.write_request(&req).await?;

        for msg in self.read_response().await? {
            match msg {
                Ok(Message::Log { .. }) => info!("skipping log response..."),
                Ok(Message::Response {
                    payload, status, ..
                }) => match payload {
                    ResponsePayload::LogConfig(LogConfigResponse::RetrieveIdRanges {
                        log_mask_sizes,
                    }) => {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(log_mask_sizes);
                    }
                    _ => info!("skipping non-LogConfigResponse response..."),
                },
                Err(e) => error!("error parsing message: {e:?}"),
            }
        }

        Err(DiagDeviceError::NoResponse(req))
    }

    async fn set_log_mask(&mut self, log_type: u32, log_mask_bitsize: u32) -> DiagResult<()> {
        let req = build_log_mask_request(
            log_type,
            log_mask_bitsize,
            &LOG_CODES_FOR_RAW_PACKET_LOGGING,
        );
        self.write_request(&req).await?;

        for msg in self.read_response().await? {
            match msg {
                Ok(Message::Log { .. }) => info!("skipping log response..."),
                Ok(Message::Response {
                    payload, status, ..
                }) => {
                    if let ResponsePayload::LogConfig(LogConfigResponse::SetMask) = payload {
                        if status != 0 {
                            return Err(DiagDeviceError::RequestFailed(status, req));
                        }
                        return Ok(());
                    }
                }
                Err(e) => error!("error parsing message: {e:?}"),
            }
        }

        Err(DiagDeviceError::NoResponse(req))
    }

    pub async fn config_logs(&mut self) -> DiagResult<()> {
        info!("retrieving diag logging capabilities...");
        let log_mask_sizes = self.retrieve_id_ranges().await?;

        for (log_type, &log_mask_bitsize) in log_mask_sizes.iter().enumerate() {
            if log_mask_bitsize > 0 {
                self.set_log_mask(log_type as u32, log_mask_bitsize).await?;
                info!("enabled logging for log type {log_type}");
            }
        }

        Ok(())
    }
}

// also found in: https://android.googlesource.com/kernel/msm.git/+/android-7.1.0_r0.3/drivers/char/diag/diagchar.h#399
//
// the code on
// https://github.com/P1sec/QCSuper/blob/master/docs/The%20Diag%20protocol.md#the-diag-protocol-over-devdiag
// is misleading, mode_param is only 8 bits. sending the larger [u32; 3] payload will cause the
// IOCTL to be rejected by TPLINK M7350 HW rev 5
//
// TPLINK M7350 v5 source code can be downloaded at https://www.tp-link.com/de/support/gpl-code/?app=omada
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DiagLoggingModeParam {
    req_mode: u32,
    peripheral_mask: u32,
    mode_param: u8,
}

// Triggers the diag device's debug logging mode
fn enable_frame_readwrite(fd: i32, mode: u32, configured_device: &Device) -> DiagResult<()> {
    unsafe {
        if libc::ioctl(fd, DIAG_IOCTL_SWITCH_LOGGING, mode, 0, 0, 0) < 0 {
            let mut try_params = vec![DiagLoggingModeParam {
                req_mode: mode,
                peripheral_mask: u32::MAX,
                mode_param: 0,
            }];
            if configured_device == &Device::Tplink {
                // tplink M7350 HW revision 3-8 need this mode
                try_params.insert(
                    0,
                    DiagLoggingModeParam {
                        req_mode: mode,
                        peripheral_mask: 0,
                        mode_param: 1,
                    },
                );
            }

            let mut ret = 0;

            for params in &try_params {
                let mut params = *params;
                ret = libc::ioctl(
                    fd,
                    DIAG_IOCTL_SWITCH_LOGGING,
                    &mut params as *mut DiagLoggingModeParam,
                    std::mem::size_of::<DiagLoggingModeParam>(),
                    0,
                    0,
                    0,
                    0,
                );
                if ret == 0 {
                    break;
                }
            }

            if ret < 0 {
                let msg = format!("DIAG_IOCTL_SWITCH_LOGGING ioctl failed with error code {ret}");
                return Err(DiagDeviceError::InitializationFailed(msg));
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
            return Err(DiagDeviceError::InitializationFailed(msg));
        }
    }
    Ok(use_mdm)
}
