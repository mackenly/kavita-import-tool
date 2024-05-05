// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
mod utils;

#[tauri::command]
fn process_files(
    _app: tauri::AppHandle,
    files: Vec<String>,
    destination: String,
    preferred: String,
    create_in_folder: bool,
) -> Vec<String> {
    let mut destination: String = destination.to_string();
    let files: Vec<String> = files
        .iter()
        .map(|file: &String| {
            println!("Processing file: {}", file);
            file.to_string()
        })
        .collect();
    // log the files to the console
    println!("{:?}", files);

    // if create_in_folder is true...
    if create_in_folder == true {
        let folder_name: String = format!(
            "kavita-import-tool-output-{}",
            chrono::Utc::now().timestamp()
        );
        let folder_path: String = format!("{}/{}", destination, folder_name);
        std::fs::create_dir_all(&folder_path).expect("Failed to create folder");
        destination = folder_path.to_string();
        println!("Created folder: {}", folder_path);
    } else {
        println!("Destination folder: {}", destination);
    }

    // order of file type priority
    let preferred_type_order: Vec<&str> = utils::get_preferred_vector(preferred);

    // go through each file and group them by file name (not considering the extension)
    let grouped_files: std::collections::HashMap<String, Vec<String>> = utils::group_files(files.clone());

    // go through each group, create a folder in the destination folder, and move one of the files (use preferred then fallback to perferred_type_order)
    for (group, files) in grouped_files.iter() {
        // only the lowest level folder name, not the full path, depending on OS separator
        let group_name: String = group
            .split(std::path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        let group_folder_path: String =
            format!("{}{}{}", destination, std::path::MAIN_SEPARATOR, group_name);
        std::fs::create_dir_all(&group_folder_path).expect("Failed to create folder");
        println!("Created folder: {}", group_folder_path);
        let mut preferred_file: Option<String> = None;
        for preferred_type in preferred_type_order.iter() {
            for file in files.iter() {
                if file.ends_with(preferred_type) {
                    preferred_file = Some(file.to_string());
                    break;
                }
            }
            if preferred_file.is_some() {
                break;
            }
        }
        let preferred_file: String = preferred_file.unwrap_or(files[0].to_string());
        let preferred_file_name: String = preferred_file
            .split(std::path::MAIN_SEPARATOR)
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        println!("Preferred file: {}", preferred_file_name);
        let preferred_file_path: String = format!(
            "{}{}{}",
            group_folder_path,
            std::path::MAIN_SEPARATOR,
            preferred_file_name
        );
        println!("Preferred file path: {}", preferred_file_path);
        std::fs::copy(preferred_file, &preferred_file_path).expect("Failed to copy file");
        println!("Copied file: {}", preferred_file_path);
    }
    files
}

#[tauri::command]
async fn open_file_location(
    _app: tauri::AppHandle,
    _window: tauri::Window,
    file_path: String,
) -> bool {
    // open the file location
    let platform: &str = std::env::consts::OS;
    match platform {
        "macos" => {
            Command::new("open")
                .arg(file_path)
                .spawn()
                .expect("Failed to open file location");
        }
        "windows" => {
            let _ = Command::new("explorer")
                .arg(file_path)
                .output()
                .expect("Failed to open file location");
        }
        "linux" => {
            Command::new("xdg-open")
                .arg(file_path)
                .spawn()
                .expect("Failed to open file location");
        }
        _ => {
            println!("Unsupported platform: {}", platform);
        }
    }
    true
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_files, open_file_location])
        //.invoke_handler(tauri::generate_handler![process_files])
        //.invoke_handler(tauri::generate_handler![open_file_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}