#![feature(setgroups)]

//! a simple shell for uploading to the orbic device.
//! 
//! It literally just runs bash as UID/GID 0 
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

const ANDROID_PARANOID_NETWORK_GROUPS: &[u32] = &[
   3001, // AID_BT
   3002, // AID_BT_NET
   3003, // AID_INET
   3004, // AID_NET_RAW
   3005, // AID_ADMIN
];

fn main() {
   let mut args = env::args();

   // discard argv[0]
   let _ = args.next();
   Command::new("/bin/bash")
	.args(args)
	.uid(0)
	.gid(0)
   .groups(ANDROID_PARANOID_NETWORK_GROUPS)
	.exec();
}
