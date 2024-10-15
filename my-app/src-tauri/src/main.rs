// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod excel_handler;
mod scraper;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      excel_handler::select_excel_file,
      excel_handler::read_excel_file,
      scraper::scraper,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
