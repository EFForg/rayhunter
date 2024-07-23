//! a simple shell for uploading to the orbic device.
//! 
//! It literally just runs bash as UID/GID 0 
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

use nix::unistd::{Gid, Uid};

fn main() {
   let mut args = env::args();

   nix::unistd::setegid(Gid::from_raw(0)).expect("setegid(0) failed");
   nix::unistd::seteuid(Uid::from_raw(0)).expect("seteuid(0) failed");

   // discard argv[0]
   let _ = args.next();
   Command::new("/bin/bash")
	.args(args)
	.uid(0)
	.gid(0)
	.exec();
}
