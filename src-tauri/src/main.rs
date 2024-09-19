// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;

use tauri::command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[command]
fn convert(file: &str, path: &str) -> String {
    // let file_path = Path::new(file);
    utils::convert(file, path);
    String::from("ok")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
