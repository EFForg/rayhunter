//! a simple shell for uploading to the orbic device.
//! 
//! It literally just runs bash as UID/GID 0 
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

fn main() {
   let mut args = env::args();

   // discard argv[0]
   let _ = args.next();
   Command::new("/bin/bash")
	.args(args)
	.uid(0)
	.gid(0)
	.exec();
}
