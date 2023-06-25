use serde_json::json;
use smoke::Signal;
use tauri::Manager;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
  data::UserIdentifier,
  frontend::{notification, os_notify, Message},
  network::{peer_tunnel::runtime::PeerTunnelRuntime, Networking},
};

use super::msg_action::Action;

pub trait CampfireMessage {
  fn try_campfire(&mut self, message: &str) -> Action {
    match message.chars().next() {
      Some(':') => {
        self.parse(message);
        Action::Capture
      }
      _ => Action::Pass,
    }
  }

  fn parse(&mut self, message: &str) {
    let mut args = message.split_whitespace();
    match args.next() {
      Some(":campfire") => {
        // campfire id is "campfire:{name}:{participants}"
        if let Some(campfire_id) = args.next() {
          // returns what would be left if we re-concat the rest of args but unmodified (whitespaces stay intact)
          let msg = message
            .split_once(' ')
            .expect("string was validated before")
            .1
            .split_once(' ')
            .expect("string was validated before2")
            .1;

          self.emit_campfire_msg(msg, campfire_id);
        } else {
          tracing::error!("no campfire id");
        }
      }
      Some(":campfire-new") => {
        // campfire id is "campfire:{name}:{participants}"
        if let Some(campfire_id) = args.next() {
          self.new_campfire(campfire_id);
        } else {
          tracing::error!("no campfire id");
        }
      }
      _ => (),
    }
  }

  fn emit_campfire_msg(&self, message: &str, campfire_id: &str);
  fn new_campfire(&self, campfire_id: &str);
}

impl<T> CampfireMessage for PeerTunnelRuntime<T>
where
  T: AsyncRead + AsyncWrite + Unpin,
{
  fn emit_campfire_msg(&self, msg: &str, campfire_id: &str) {
    let message = Message {
      msg,
      sender: self.peer(),
    };

    self
      .window()
      .emit(&format!("user_msg_{campfire_id}"), message)
      .expect("Failed to emit event");

    os_notify(notification().title("campfire message").body(msg));
  }

  fn new_campfire(&self, campfire_id: &str) {
    let users = campfire_id
      .split_once(':')
      .expect("invalid campfire_id")
      .1
      .split_once(':')
      .expect("invalid campfire_id2")
      .1
      .split('-')
      .map(|string| UserIdentifier {
        bs58: string.into(),
      })
      .collect::<Vec<_>>();

    self
      .window()
      .emit("new-campfire", json!({ "id": campfire_id }))
      .expect("Failed to emit event");

    let window = self.window().clone();
    let id = campfire_id.to_string();
    let me = self.me().clone();
    let handler = self
      .window()
      .listen(format!("send_message_{id}"), move |e| {
        let msg = serde_json::from_str::<Signal>(
          e.payload()
            .expect("Invalid payload in send_message_<id> event"),
        )
        .expect("Invalid Json inside of payload from send_message_<id> event");

        let msg = match msg {
          Signal::Message(msg) => {
            format!(":campfire {id} {msg}")
          }
          _ => {
            let discriminant = std::mem::discriminant(&msg);
            tracing::warn!("cannot send non msg {discriminant:?} message in campfire");
            return;
          }
        };

        let networking = window.state::<Networking>();
        let mutex_guard = networking.chats.lock().expect("poisoned mutex");

        for user in users.iter() {
          if *user == me {
            continue;
          }

          let Some(tunnel) = mutex_guard.values().find(|&room| room.peer_id() == user) else{
            tracing::warn!("campfire cannot send msg: not connected to peer: {user:?}");
            continue;
          };

          let sender = tunnel.sender().clone();
          let msg = msg.clone();
          tokio::spawn(async move {
            if let Err(err) = sender.send(Signal::Message(msg)).await {
              tracing::warn!("error sending campfire message to a tunnel: {err}");
            }
          });
        }
      });
  }
}
