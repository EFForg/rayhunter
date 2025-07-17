use core::str;
use std::path::Path;
use std::process::exit;

fn main() {
    println!("cargo::rerun-if-env-changed=NO_FIRMWARE_BIN");
    let include_dir = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../target/armv7-unknown-linux-musleabihf/firmware/"
    ));
    set_binary_var(include_dir, "FILE_ROOTSHELL", "rootshell");
    set_binary_var(include_dir, "FILE_RAYHUNTER_DAEMON", "rayhunter-daemon");
}

fn set_binary_var(include_dir: &Path, var: &str, file: &str) {
    if std::env::var_os("NO_FIRMWARE_BIN").is_some() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        std::fs::create_dir_all(&out_dir).unwrap();
        let blank = Path::new(&out_dir).join("blank");
        std::fs::write(&blank, []).unwrap();
        println!("cargo::rustc-env={var}={}", blank.display());
        return;
    }
    if std::env::var_os(var).is_none() {
        let binary = include_dir.join(file);
        if !binary.exists() {
            println!(
                "cargo::error=Firmware binary {file} not present at {}",
                binary.display()
            );
            exit(0);
        }
        println!("cargo::rustc-env={var}={}", binary.display());
        println!("cargo::rerun-if-changed={}", binary.display());
    }
}
