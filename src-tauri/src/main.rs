#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

mod network;
use log::trace;
use network::ctrl_chnl::{connect, requests::*, responses::*, State};
use network::{chat_exists, Networking};
use tokio::sync::RwLock;

fn main() {
  env_logger::init();

  trace!("Running as: {}", dotenv!("PUBLIC_KEY"));

  tauri::Builder::default()
    // Application State
    .manage(Networking {
      chats: Default::default(),
      pending: Default::default(),
    })
    .manage(RwLock::<Option<State>>::new(None))
    // Tauri Commands
    .invoke_handler(tauri::generate_handler![
      chat_exists,
      connect,
      request_room,
      accept_room,
    ])
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::Focused(focused) => {
        // dev pls add atomic bool here
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
