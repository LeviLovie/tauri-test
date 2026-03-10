use anyhow::{Context, Result};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
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
