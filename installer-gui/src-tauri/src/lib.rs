use tauri::Emitter;

async fn run_installer(app_handle: tauri::AppHandle, args: String) -> anyhow::Result<()> {
    tauri::async_runtime::spawn_blocking(move || {
        installer::run_with_callback(
            // TODO: we should split using something similar to shlex in python
            args.split_whitespace().map(String::from).collect(),
            Some(Box::new(move |output| {
                app_handle
                    .emit("installer-output", output)
                    .expect("Error sending Rayhunter CLI installer output to GUI frontend");
            })),
        )
    })
    .await?
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
