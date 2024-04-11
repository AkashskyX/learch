
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::from_utf8;

use sysinfo::{System, SystemExt, DiskExt};
use tantivy::Index ;

use open;
mod search_index;



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
fn get_index_metadata(index_path: String) -> Result<String, String> {
    search_index::get_index_operational_data(index_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_files(query: String, index_path: String) -> Result<Vec<String>, String> {
    let index = Index::open_in_dir(&index_path).map_err(|e| e.to_string())?;

    search_index::index_search(&index, &query)
        .map_err(|e| e.to_string())
}






#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    std::fs::canonicalize(&path)
        .map_err(|e| e.to_string())
        .and_then(|path| {
            if path.is_dir() {
                open::that(&path)
                    .map(|_| ())  // Convert Ok(ExitStatus) to Ok(())
                    .map_err(|e| e.to_string())
            } else {
                Err("Path is not a directory".into())
            }
        })
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




#[tauri::command]
async fn create_and_index(root_path: String , app: tauri::AppHandle) -> Result<(), String> {
    let index = search_index::create_index(&app).map_err(|e| e.to_string())?;
        // Call the function to add documents to the index
    search_index::index_files(app, &index, &root_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_index_path(app: tauri::AppHandle) -> Result<String, String> {
    // Get the app data directory
    let app_data_dir = tauri::api::path::app_data_dir(&app.config())
        .ok_or("Unable to locate app data directory")?;

    // Append 'index_data' to the path
    let index_path = app_data_dir.join("index_data");

    // Convert the path to a String and return it
    Ok(index_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn delete_index_command(app: tauri::AppHandle) -> Result<(), String> {
    // Logic to delete the index
    // This usually involves deleting the files in the index directory
    let app_data_dir = tauri::api::path::app_data_dir(&app.config())
        .ok_or_else(|| "Unable to locate app data directory".to_string())?;
    let index_path = app_data_dir.join("index_data");
    if index_path.exists() {
        std::fs::remove_dir_all(&index_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_disk_info  , search_files , list_files_in_directory , open_file  ,open_folder , get_index_metadata,  create_and_index , get_index_path , delete_index_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
