#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

mod data;
mod embed;
mod network;

pub mod frontend;

use std::sync::atomic::AtomicBool;

use data::tauri::*;
use embed::embed;
use network::ctrl_chnl::{connect, requests::*, responses::*, State};
use network::{chat_exists, Networking};
use once_cell::sync::Lazy;
use std::sync::atomic::Ordering;
use tokio::sync::RwLock;
use tracing::trace;

pub static FOCUS: AtomicBool = AtomicBool::new(false);

pub static APPID: Lazy<String> = Lazy::new(|| {
  tauri::generate_context!()
    .config()
    .tauri
    .bundle
    .identifier
    .clone()
});

fn main() {
  #[cfg(not(feature = "tracing"))]
  tracing_subscriber::fmt::init();
  #[cfg(feature = "tracing")]
  console_subscriber::init();

  #[cfg(feature = "certgen")]
  {
    trace!(concat!("emberry certgen v", env!("CARGO_PKG_VERSION")));
    generate_user_certificate();
    return;
  }

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
      update_username,
      get_local,
      embed,
      generate_user_certificate,
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
