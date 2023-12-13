
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{System, SystemExt, DiskExt};
use std::str::from_utf8;


#[tauri::command]
fn get_disk_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disk_info = sys.disks().iter()
        .map(|disk| {
            let file_system = from_utf8(disk.file_system())
                .unwrap_or("Invalid UTF-8");

            format!(
                "Disk {} ({}): {:.1} GB / {:.1} GB",
                disk.name().to_str().unwrap_or("Unknown"),
                file_system,
                disk.available_space() as f64 / 1_000_000_000.0,
                disk.total_space() as f64 / 1_000_000_000.0
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    disk_info
}



#[tauri::command]
fn list_files_in_directory(path: String) -> Result<Vec<(String, bool)>, String> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(&path);
    if !path.is_dir() {
        return Err("The provided path is not a directory".to_string());
    }

    let mut entries = Vec::new();
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir.flatten() {
                let file_type = entry.file_type().map_err(|_| "Failed to read file type")?;
                let is_dir = file_type.is_dir();
                let file_name = entry.file_name().into_string().map_err(|_| "Failed to read file name")?;
                entries.push((file_name, is_dir));
            }
            Ok(entries)
        }
        Err(_) => Err("Failed to read the directory".to_string()),
    }
}







fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_disk_info , list_files_in_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}