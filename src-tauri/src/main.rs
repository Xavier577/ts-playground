// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn run_ts_code(code: String) -> Result<String, String> {
    let output = Command::new("ts-node")
        .arg("-e")
        .arg(code)
        .output()
        .expect("failed to execute");

    if output.status.success() {
        let result = String::from_utf8(output.stdout).unwrap();
        Ok(result)
    } else {
        let error = String::from_utf8(output.stderr).unwrap();
        Err(error)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_ts_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
