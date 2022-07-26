#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

#[cfg(not(target_os = "linux"))]
use tauri::Manager;

#[cfg(not(target_os = "linux"))]
mod window;

mod network;
use network::{
  chat_exists,
  hole_punch, Networking,
};
use network::ctrl_chnl::{connect, responses::*, State};
use tokio::sync::RwLock;
use tauri_plugin_store::PluginBuilder;

fn main() {
  let builder = tauri::Builder::default();

  #[cfg(not(target_os = "linux"))]
  let builder = builder.setup(|app| {
    let window = app.get_window("main").unwrap();

    window::set_shadow(&window, true).expect("Unsupported platform!");

    // #[cfg(target_os = "windows")]
    // if let Err(_) = window_vibrancy::apply_mica(&window) {
    //   window_vibrancy::apply_blur(&window, Some((32, 32, 32, 255))).unwrap();
    // }

    Ok(())
  });

  builder
    .manage(Networking {
      chats: Default::default(),
      pending: Default::default(),
    })
    .manage(
      RwLock::<Option<State>>::new(None)
    )
    .plugin(PluginBuilder::default().build())
    .invoke_handler(tauri::generate_handler![
      toggle_devtools,
      hole_punch,
      chat_exists,
      connect,
      send_room_affirm,
    ])
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
