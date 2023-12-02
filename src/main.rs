use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use bytes::{Buf, BufMut};
use std::os::fd::AsRawFd;
use std::thread;
use thiserror::Error;

type DiagResult<T> = Result<T, DiagDeviceError>;

const BUFFER_LEN: usize = 1024 * 1024 * 10;
const USER_SPACE_DATA_TYPE: i32 = 32;
const DIAG_IOCTL_REMOTE_DEV: u32 = 32;
const MEMORY_DEVICE_MODE: i32 = 2;
const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;

#[derive(Error, Debug)]
enum DiagDeviceError {
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to initialize /dev/diag: {0}")]
    InitializationFailed(String),
    #[error("Failed to read diag device: {0}")]
    DeviceReadFailed(String),
}

struct DiagDevice {
    file: File,
    use_mdm: i32,
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
            use_mdm,
        })
    }

    pub fn try_clone(&self) -> DiagResult<Self> {
        Ok(DiagDevice {
            file: self.file.try_clone()?,
            use_mdm: self.use_mdm,
        })
    }

    pub fn read_response(&mut self) -> DiagResult<Option<Vec<Vec<u8>>>> {
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
            messages.push(msg);
        }

        Ok(Some(messages))
    }

    pub fn write_request(&mut self, req: &[u8]) -> DiagResult<()> {
        let mut buf: Vec<u8> = vec![];
        buf.put_i32_le(USER_SPACE_DATA_TYPE);
        if self.use_mdm > 0 {
            buf.put_i32_le(-1);
        }
        buf.extend_from_slice(req);
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
}

fn main() -> std::io::Result<()> {
    println!("Starting server");
    let listener = TcpListener::bind("0.0.0.0:43555")?;

    // Since we only care about one client at a time, store a copy of that
    // client's TcpStream in a mutex. This lets us write to the client from a
    // separate thread
    let client_mutex: Arc<Mutex<Option<TcpStream>>> = Arc::new(Mutex::new(None));

    // initialize the diag device and create a cloned handle to its file. this
    // lets us perform reads and writes in separate threads. i *think* this is
    // sound
    let mut dev_reader = DiagDevice::new().unwrap();
    let mut dev_writer = dev_reader.try_clone().unwrap();

    // Spawn a thread to continuously read from the diag device, sending any
    // messages to the client
    let client_mutex_clone = client_mutex.clone();
    thread::spawn(move || {
        loop {
            match dev_reader.read_response() {
                Ok(Some(msgs)) => {
                    if let Some(client_writer) = client_mutex_clone.lock().unwrap().as_mut() {
                        println!("> Writing {} diag messages to client", msgs.len());
                        for msg in msgs {
                            client_writer.write_all(&msg).unwrap();
                        }
                    }
                },
                Ok(None) => {},
                Err(err) => {
                    println!("Unable to read from /dev/diag: {}", err);
                    return;
                },
            }
        }
    });

    // Accept connections from a client (only one is accepted at a time),
    // writing any data received to the diag device
    loop {
        println!("Waiting for client");
        let (mut client_reader, _) = listener.accept()?;

        println!("Client connected");
        let client_writer = client_reader.try_clone()?;
        {
            let mut client_writer_mutex = client_mutex.lock().unwrap();
            *client_writer_mutex = Some(client_writer);
        }

        let mut buf = vec![0; BUFFER_LEN];
        loop {
            let bytes_read = client_reader.read(&mut buf).unwrap();
            if bytes_read == 0 {
                println!("Client disconnected");
                {
                    let mut client_writer_mutex = client_mutex.lock().unwrap();
                    *client_writer_mutex = None;
                }
                break;
            }
            println!("< Got {} bytes from client", bytes_read);
            dev_writer.write_request(&buf[0..bytes_read]).unwrap();
        }
    }
}
