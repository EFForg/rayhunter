use std::path::Path;

fn main() {
    println!("cargo::rerun-if-env-changed=FIRMWARE_PROFILE");
    let profile = std::env::var("FIRMWARE_PROFILE").unwrap_or_else(|_| {
        // Default to firmware-devel for debug builds, firmware for release builds
        if std::env::var("PROFILE").as_deref() == Ok("release") {
            "firmware".to_string()
        } else {
            "firmware-devel".to_string()
        }
    });
    let include_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../target/armv7-unknown-linux-musleabihf")
        .join(&profile);
    set_binary_var(&include_dir, "FILE_ROOTSHELL", "rootshell");
    set_binary_var(&include_dir, "FILE_RAYHUNTER_DAEMON", "rayhunter-daemon");

    let wpa_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tools/build-wpa-supplicant/out");
    set_binary_var(&wpa_dir, "FILE_WPA_SUPPLICANT", "wpa_supplicant");
    set_binary_var(&wpa_dir, "FILE_WPA_CLI", "wpa_cli");
    set_binary_var(&wpa_dir, "FILE_IW", "iw");
}

fn set_binary_var(include_dir: &Path, var: &str, file: &str) {
    println!("cargo::rerun-if-env-changed={var}");
    if std::env::var_os(var).is_some() {
        return;
    }
    let binary = include_dir.join(file);
    println!("cargo::rerun-if-changed={}", binary.display());
    if binary.exists() {
        println!("cargo::rustc-env={var}={}", binary.display());
    } else {
        println!(
            "cargo::warning=Firmware binary {file} not present at {}; \
             installers that need it will fail",
            binary.display()
        );
        println!("cargo::rustc-env={var}=");
    }
}
