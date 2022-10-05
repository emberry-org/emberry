#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

mod network;
use network::ctrl_chnl::{connect, requests::*, responses::*, State};
use network::{chat_exists, Networking};
use tokio::sync::RwLock;

fn main() {
  env_logger::init();

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
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
