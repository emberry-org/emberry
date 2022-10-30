#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

mod data;
mod embed;
mod history;
mod network;

use std::sync::atomic::AtomicBool;

use data::tauri::*;
use embed::embed;
use log::trace;
use network::ctrl_chnl::{connect, requests::*, responses::*, State};
use network::{chat_exists, Networking};
use std::sync::atomic::Ordering;
use tokio::sync::RwLock;

pub static FOCUS: AtomicBool = AtomicBool::new(false);

fn main() {
  env_logger::init();

  trace!(concat!("emberry-rs v", env!("CARGO_PKG_VERSION")));
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
      get_usr_info,
      get_usrs,
      embed,
    ])
    // TEMP / TODO : This will be obsolete once the `window.is_focused()` function is released from Tauri.
    .on_window_event(|event| {
      if let tauri::WindowEvent::Focused(focused) = event.event() {
        FOCUS.store(*focused, Ordering::SeqCst);
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
