//! a simple shell for uploading to the orbic device.
//! 
//! It literally just runs bash as UID/GID 0, with special Android GIDs 3003
//! (AID_INET) and 3004 (AID_NET_RAW).
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::env;

#[cfg(target_arch = "arm")]
use nix::unistd::Gid;

fn main() {
   let mut args = env::args();

   // Android's "paranoid network" feature restricts network access to
   // processes in specific groups. More info here:
   // https://www.elinux.org/Android_Security#Paranoid_network-ing
   #[cfg(target_arch = "arm")] {
      let gids = &[
         Gid::from_raw(3003), // AID_INET
         Gid::from_raw(3004), // AID_NET_RAW
      ];
      nix::unistd::setgroups(gids).expect("setgroups failed");
   }

   // discard argv[0]
   let _ = args.next();
   // This call will only return if there is an error
   let error = Command::new("/bin/bash")
	.args(args)
	.uid(0)
	.gid(0)
	.exec();
   eprintln!("Error running command: {error}");
   std::process::exit(1);
}
