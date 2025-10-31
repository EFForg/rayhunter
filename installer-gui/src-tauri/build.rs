use std::env::consts::EXE_SUFFIX;
use std::path::PathBuf;

fn main() {
    println!("cargo::rerun-if-env-changed=INSTALLER_PATH");
    println!("cargo::rerun-if-env-changed=SKIP_INSTALLER_COPY");

    let destination = get_installer_destination();
    if std::env::var_os("SKIP_INSTALLER_COPY").is_none() {
        let cli_installer =
            std::env::var_os("INSTALLER_PATH").map_or_else(default_installer_path, PathBuf::from);
        if !cli_installer.exists() {
            println!(
                "cargo::error=CLI installer binary not present at {}",
                cli_installer.display()
            );
            std::process::exit(0);
        }
        std::fs::copy(&cli_installer, &destination).unwrap();
        println!("cargo::rerun-if-changed={}", cli_installer.display());
        println!("cargo::rerun-if-changed={}", destination.display());
    } else if !destination.exists() {
        // if SKIP_INSTALLER_COPY is set, make sure something exists at destination so the build succeeds
        std::fs::write(&destination, []).unwrap();
    }

    tauri_build::build()
}

fn default_installer_path() -> PathBuf {
    // the approach used here was taken from https://github.com/rust-lang/cargo/issues/9661#issuecomment-1722358176
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let profile = std::env::var("PROFILE").unwrap();
    let profile_dir = out_dir
        .ancestors()
        .find(|&path| path.ends_with(&profile))
        .unwrap();

    profile_dir.join(format!("installer{EXE_SUFFIX}"))
}

fn get_installer_destination() -> PathBuf {
    // tauri expects included binaries to have the target triple appended to the file name like
    // this. see https://tauri.app/develop/sidecar/
    let target_triple = std::env::var("TARGET").unwrap();
    [
        "binaries",
        &format!("installer-cli-{target_triple}{EXE_SUFFIX}"),
    ]
    .iter()
    .collect()
}
