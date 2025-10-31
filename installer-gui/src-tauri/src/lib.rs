use anyhow::Context;
use tauri::Emitter;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

async fn run_installer(app_handle: tauri::AppHandle, args: String) -> anyhow::Result<()> {
    let (mut rx, _child) = app_handle
        .shell()
        .sidecar("installer-cli")
        .context("Error preparing Rayhunter CLI installer to be run")?
        .args(args.split_whitespace())
        .spawn()
        .context("Error launching Rayhunter CLI installer")?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line_bytes) | CommandEvent::Stderr(line_bytes) => {
                let line = String::from_utf8(line_bytes)
                    .context("Error parsing Rayhunter CLI installer output")?;
                app_handle
                    .emit("installer-output", &line)
                    .context("Error sending Rayhunter CLI installer output to GUI frontend")?;
            }
            _ => (),
        };
    }
    Ok(())
}

#[tauri::command]
async fn install_rayhunter(app_handle: tauri::AppHandle, args: String) -> Result<(), String> {
    // the return value of tauri commands needs to be serializable by serde which we accomplish
    // here by converting anyhow::Error to a string
    run_installer(app_handle, args)
        .await
        .map_err(|error| format!("{error:?}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![install_rayhunter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
