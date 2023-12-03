mod hdlc;
mod diag;

use crate::hdlc::{hdlc_encapsulate, hdlc_decapsulate};
use crate::diag::{DiagDevice};

fn main() -> std::io::Result<()> {
    let mut dev = DiagDevice::new().unwrap();
    dev.config_logs().unwrap();

    loop {
        let msgs = dev.read_response().unwrap();
        if let Some(msgs) = msgs {
            for msg in msgs {
                println!("msg: {:?}", msg);
            }
        }
    }
}
