use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;

#[tauri::command]
fn get_base_path(app: AppHandle) -> Result<String, String> {
    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_base_path])
        .setup(|app| {
            let scope = app.fs_scope();
            let base_path = app
                .path()
                .app_data_dir()
                .context("failed to get app data directory")?;
            std::fs::create_dir_all(&base_path).context("failed to create base directory")?;
            scope
                .allow_directory(base_path, false)
                .context("failed to allow base directory")?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .context("failed to run tauri application")
}
