#[tauri::command]
pub fn save_history() {
  // Get the AppData directory for emberry.
  let config = tauri::Config::default();
  println!("{}Emberry\\", tauri::api::path::app_dir(&config).unwrap().to_str().unwrap());
}