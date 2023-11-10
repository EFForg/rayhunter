use std::io::Cursor;
use std::mem;
use std::io;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

type DiagResult<T> = Result<T, DiagDeviceError>;

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const USER_SPACE_DATA_TYPE: i32 = 32;
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

#[derive(Error, Debug)]
enum DiagDeviceError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(String),
}

struct DiagDevice {
    pub file: File,
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
    pub async fn new() -> DiagResult<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .open("/dev/diag").await?;
        let fd = file.as_raw_fd();

        enable_frame_readwrite(fd, MEMORY_DEVICE_MODE)?;
        let use_mdm = determine_use_mdm(fd)?;

        Ok(DiagDevice {
            file,
            use_mdm,
        })
    }

    pub async fn read_response(&mut self) -> DiagResult<Option<Vec<Vec<u8>>>> {
        let mut buf = vec![0; BUFFER_LEN];
        let bytes_read = self.file.read(&mut buf).await?;
        if bytes_read < 4 {
            let msg = format!("read {} bytes from diag device, expected > 4", bytes_read);
            return Err(DiagDeviceError::DeviceReadFailed(msg));
        }
        let mut reader = Cursor::new(buf);

        // is this a USER_SPACE_DATA_TYPE?
        if reader.read_i32().await? != USER_SPACE_DATA_TYPE {
            return Ok(None);
        }

        let num_messages = reader.read_u32().await?;
        let mut messages = Vec::new();

        for _ in 0..num_messages {
            let msg_len = reader.read_u32().await? as usize;
            let mut msg = vec![0; msg_len];
            reader.read_exact(&mut msg).await?;
            messages.push(msg);
        }

        Ok(Some(messages))
    }

    pub async fn write_request(&mut self, req: &[u8]) -> DiagResult<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(req.len());
        buf.write_i32(USER_SPACE_DATA_TYPE).await?;
        if self.use_mdm > 0 {
            buf.write_u32(0xffffffff).await?;
        }
        buf.extend_from_slice(req);
        self.file.write_all(&buf).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Initializing DIAG");
    let dev = Arc::new(Mutex::new(DiagDevice::new().await.unwrap()));
    let clients: Arc<Mutex<Vec<OwnedWriteHalf>>> = Arc::new(Mutex::new(Vec::new()));

    let dev_clone = dev.clone();
    let clients_clone = clients.clone();
    tokio::spawn(async move {
        loop {
            let mut dev_ = dev_clone.lock().await;
            match dev_.read_response().await.unwrap() {
                Some(msg) => {
                    let mut clients_ = clients_clone.lock().await;
                    for client in clients_.iter_mut() {
                        for buf in &msg {
                            client.write(buf).await.unwrap();
                        }
                    }
                },
                None => {},
            }
        }
    });

    println!("Starting server");
    let listener = TcpListener::bind("0.0.0.0:1312").await?;

    // handle incoming clients
    loop {
        let (socket, _) = listener.accept().await?;
        let (mut read, write) = socket.into_split();
        let client_idx: usize;
        {
            let mut clients_ = clients.lock().await;
            clients_.push(write);
            client_idx = clients_.len();
        }
        let dev_clone = dev.clone();
        let clients_clone = clients.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; BUFFER_LEN];
            loop {
                let bytes_read = read.read(&mut buf).await.unwrap();
                if bytes_read == 0 {
                    let mut clients_ = clients_clone.lock().await;
                    clients_.remove(client_idx);
                    println!("client {} disconnected", client_idx);
                    break;
                }
                println!("waiting to write {} byte diag request...", bytes_read);
                let mut dev_ = dev_clone.lock().await;
                dev_.write_request(&buf[0..bytes_read]).await.unwrap();
                println!("diag request complete");
            }
        });
    }
}
