use std::fs::File;
use std::mem;
use std::os::unix::io::AsRawFd;
use thiserror::Error;

type DiagResult<T> = Result<T, DiagDeviceError>;

const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

#[derive(Error, Debug)]
enum DiagDeviceError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
}

struct DiagDevice {
    file: File,
    fd: i32,
    use_mdm: i32,
}

fn enable_frame_readwrite(fd: i32, mode: i32) -> DiagResult<()> {
    unsafe {
        if libc::ioctl(fd, DIAG_IOCTL_SWITCH_LOGGING, mode, 0, 0, 0) < 0 {
            let ret = libc::ioctl(
                fd,
                DIAG_IOCTL_SWITCH_LOGGING,
                &mut [mode, -1, 0] as *mut _, // diag_logging_mode_param_t
                mem::size_of::<[i32; 3]>(), 0, 0, 0, 0
            );
            if ret < 0 {
                let msg = format!("DIAG_IOCTL_SWITCH_LOGGING ioctl failed with error code {}", ret);
                return Err(DiagDeviceError::InitializationFailed(msg))
            }
        }
    }
    Ok(())
}

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
            fd,
            use_mdm,
        })
    }
}

fn main() -> DiagResult<()> {
    println!("Initializing DIAG");
    let dev = DiagDevice::new()?;
    Ok(())
}
