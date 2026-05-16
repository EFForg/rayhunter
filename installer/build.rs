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
    // We need to rerun the build script if the file starts appearing or disappearing, to emit the
    // right warnings and change the envvar's value. We don't really need to rerun the build script
    // if the file changes contents.
    watch_file(&binary);
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

/// Rerun the build script if the file changes or it appears, or disappears.
///
/// Simply emitting rerun-if-changed for a nonexistent filepath (such as wpa_supplicant) will make
/// cargo recompile everything all the time. Therefore, if the file does not exist we need to watch
/// the first parent directory that does.
fn watch_file(mut file: &Path) {
    while !file.exists()
        && let Some(parent) = file.parent()
    {
        file = parent;
    }

    println!("cargo::rerun-if-changed={}", file.display());
}
