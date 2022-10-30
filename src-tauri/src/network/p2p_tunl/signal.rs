use std::{borrow::Borrow, io, sync::atomic::Ordering};

use crate::data::{
  sqlite::{exec, user::upsert},
  IdentifiedUserInfo, UserIdentifier,
};

use smoke::Signal;
use tauri::{api::notification::Notification, AppHandle, Window};

use super::p2p_loop::EventNames;

#[derive(Clone, serde::Serialize)]
struct MessageRecievedPayload<'a> {
  message: &'a Signal,
}

#[derive(Clone, serde::Serialize)]
struct UsernameChangedPayload<'a> {
  username: &'a str,
}

pub async fn handle_signal(
  signal: &Signal,
  spawn_window: &Window,
  app_handle: &AppHandle,
  events: &EventNames,
  msg_from: &mut String,
  cache: &mut IdentifiedUserInfo<'_>,
) -> Result<(), io::Error> {
  match signal {
    Signal::Username(name) => {
      if &cache.info.username != name {
        cache.info.username = name.to_string();
        let input = (cache.borrow(), |info: &IdentifiedUserInfo| {
          *msg_from = format!("Message from {}", cache.info.username);
          emit_username(spawn_window, &events.usr_name, &info.info.username)
        });
        exec(upsert, input)?;
      }
    }
    Signal::Chat(text) => {
      emit_msg(spawn_window, &events.msg_recv, signal);

      /* Create a new notification for the message */
      if !crate::FOCUS.load(Ordering::SeqCst) {
        Notification::new(&app_handle.config().tauri.bundle.identifier)
          .title(msg_from.as_str())
          .body(text)
          .show()
          .expect("Failed to send desktop notification");
      }
    }
    _ => emit_msg(spawn_window, &events.msg_recv, signal),
  }

  Ok(())
}

#[inline]
fn emit_username(window: &Window, event_name: &str, name: &str) {
  if let Err(err) = window.emit(event_name, name) {
    log::error!("Failed to emit event: '{}'", err);
  }
}

#[inline]
fn emit_msg(window: &Window, event_name: &str, signal: &Signal) {
  window
    .emit(event_name, MessageRecievedPayload { message: signal })
    .expect("Failed to emit event")
}
