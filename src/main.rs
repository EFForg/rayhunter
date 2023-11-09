use std::fs::File;
use std::mem;
use std::os::unix::io::AsRawFd;

fn main() -> std::io::Result<()> {
    const DIAG_IOCTL_SWITCH_LOGGING: u32 = 7;
    const MEMORY_DEVICE_MODE: i32 = 2;
    const DIAG_IOCTL_REMOTE_DEV: u32 = 32;

    let mut mode_param: [i32; 3] = [MEMORY_DEVICE_MODE, -1, 0]; // diag_logging_mode_param_t
    let use_mdm: i32 = 0;

    println!("Initializing DIAG");

    let diag_file = File::options()
         .read(true)
         .write(true)
         .open("/dev/diag")?;
    let diag_fd = diag_file.as_raw_fd();

    unsafe {
        if libc::ioctl(diag_fd, DIAG_IOCTL_SWITCH_LOGGING, MEMORY_DEVICE_MODE, 0, 0, 0) < 0
            && libc::ioctl(
                diag_fd,
                DIAG_IOCTL_SWITCH_LOGGING,
                &mut mode_param as *mut _,
                mem::size_of::<[i32; 3]>(), 0, 0, 0, 0
            ) < 0
        {
            println!("ioctl failed 1");
            //std::process::exit(1);
        }
    }

    unsafe {
        if libc::ioctl(diag_fd, DIAG_IOCTL_REMOTE_DEV, &use_mdm as *const i32) < 0 {
            println!("ioctl failed 2");
            std::process::exit(1);
        }
    }

    println!("successfully opened diag device");

    Ok(())
}
