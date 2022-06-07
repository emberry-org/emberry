#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![toggle_devtools])
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