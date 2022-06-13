#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

use tauri::Manager;

mod window;
use window::set_shadow;

mod network;
use network::hole_punch;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      set_shadow(&window, true).expect("Unsupported platform!");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![toggle_devtools, hole_punch])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn toggle_devtools(window: tauri::Window) {
  if window.is_devtools_open() {
    window.close_devtools();
  } else {
    window.open_devtools();
  }
}