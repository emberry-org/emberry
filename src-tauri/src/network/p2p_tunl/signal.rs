use std::{borrow::Borrow, io};

use crate::{
  data::{
    sqlite::{try_exec, user::upsert},
    IdentifiedUserInfo,
  },
  frontend::{notification, os_notify},
};

use smoke::Signal;
use tauri::Window;

use tracing::{error, trace, warn};

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
  events: &EventNames,
  msg_from: &mut String,
  cache: &mut IdentifiedUserInfo<'_>,
  opt_bridge: &mut Option<vlink::TcpBridge>,
) -> Result<(), io::Error> {
  match signal {
    Signal::Kap => (),
    Signal::Username(name) => {
      if &cache.info.username != name {
        cache.info.username = name.to_string();
        let input = (cache.borrow(), |info: &IdentifiedUserInfo| {
          *msg_from = format!("Message from {}", cache.info.username);
          emit_username(spawn_window, &events.usr_name, &info.info.username)
        });
        try_exec(upsert, input)?;
      }
    }
    Signal::Vlink(internal) => {
      let Some(bridge) = opt_bridge else {
        warn!("got vlink package while bridge was not available: {internal:?}");
        return Ok(());
      };

      bridge.input(internal.as_vlink()).await;
    }
    Signal::VlinkOpen(name) => {
      trace!("vlink opened by remote, name: {name}");
      spawn_window
        .emit("vlink-available", name)
        .expect("failed to emit vlink-available)");

      // TODO remove vlink hack
      emit_msg(
        spawn_window,
        &events.msg_recv,
        &Signal::Message(format!(
          "HAS OPENED A VLINK WITH NAME: \"{name}\"\n\nTYPE: \"/vlink_connect {name}\" TO ENABLE THE VLINK ON YOUR LOCAL PORT \"8080\"\nYOU CAN ALWAYS CLOSE THE CONNECTION USING: \"/vlink_close\""
        )),
      );
    }
    Signal::VlinkCut => {
      opt_bridge.take();
      trace!("dropped potential vlink bridge");
      emit_msg(
        spawn_window,
        &events.msg_recv,
        &Signal::Message("HAS REVOKED THE VLINK".to_string()),
      );
    }
    Signal::ChangeContext(new_peer_context) => todo!("create context/campfire system"),
    // CONTEXT SENSITIVE SIGNALS
    Signal::Message(text) => {
      emit_msg(spawn_window, &events.msg_recv, signal);

      os_notify(notification().title(&*msg_from).body(text));
    }
  }

  Ok(())
}

#[inline]
fn emit_username(window: &Window, event_name: &str, name: &str) {
  if let Err(err) = window.emit(event_name, name) {
    error!("Failed to emit event: '{}'", err);
  }
}

#[inline]
fn emit_msg(window: &Window, event_name: &str, signal: &Signal) {
  window
    .emit(event_name, MessageRecievedPayload { message: signal })
    .expect("Failed to emit event")
}
