use std::str::SplitWhitespace;

use serde_json::json;
use smoke::Signal;
use tauri::Manager;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::info;
use vlink::TcpBridge;

use crate::{
  data::UserIdentifier,
  network::{peer_tunnel::runtime::PeerTunnelRuntime, Networking},
};

use super::msg_action::Action;

const HELP: &str = r#"Available slash commands:
  `/help` or `/` - print this help
  `/vlink_open <port: u16> [<name: String>]` - open your local port <port> for a TCP Bridge connection
    <name> = `default`
  `/vlink_connect <port: u16> [<name: String>]` - connect to a TCP bridge opened by your peer
    <name> = `default`
  `/vlink_close` - close an existing TCP Bridge, when run after `/vlink_open` also terminates the peer end of the Bridge
  `/campfire <user1: String> <user2: String> [...] <userN: String> - create a new campfire with all the listed users (CAREFULL BETA)
"#;

pub trait SlashCommands {
  fn try_execute(&mut self, command: &str) -> Action {
    match command.chars().next() {
      Some('/') => {
        self.parse(command);
        Action::Capture
      }
      Some(_) => Action::Pass,
      None => Action::Pass,
    }
  }

  fn parse(&mut self, command: &str) {
    let mut args = command.split_whitespace();
    match args.next() {
      Some("/vlink_open") => self.vlink_open(args),
      Some("/vlink_close") => self.vlink_close(),
      Some("/vlink_connect") => self.vlink_connect(args),
      Some("/campfire") => self.campfire(args),
      None | Some(_) => self.println(HELP),
    }
  }

  fn println(&mut self, msg: &str);
  fn vlink_open(&mut self, args: SplitWhitespace);
  fn vlink_connect(&mut self, args: SplitWhitespace);
  fn campfire(&mut self, args: SplitWhitespace);
  fn vlink_close(&mut self);
}

impl<T> SlashCommands for PeerTunnelRuntime<T>
where
  T: AsyncRead + AsyncWrite + Unpin,
{
  fn println(&mut self, msg: &str) {
    self.sys_msg(msg)
  }

  fn vlink_open(&mut self, mut args: SplitWhitespace) {
    if self.opt_bridge.is_some() {
      self.println("There is already a Vlink active. Consider closing the existing Vlink before creating a new one");
      return;
    };

    let Some(Ok(port)) = args.next().map(|port_str| port_str.parse::<u16>()) else {
      self.println("Invalid arguments for `/vlink_open`");
      self.println(HELP);
      return;
    };

    let name = args.next().unwrap_or("default");

    self.opt_bridge = Some(TcpBridge::emit_to(port));
    info!("opened emitting TcpBridge");
    self.println(&format!("You opened a Vlink at your local port tcp:{port}"));

    let msg = Signal::VlinkOpen(name.to_string());

    self.schedule_io(msg);
  }

  fn vlink_connect(&mut self, mut args: SplitWhitespace) {
    if self.opt_bridge.is_some() {
      self.println("There is already a Vlink active. Consider closing the existing Vlink before connecting to a new one");
      return;
    };

    let Some(Ok(port)) = args.next().map(|port_str| port_str.parse::<u16>()) else {
      self.println("Invalid arguments for `/vlink_connect`");
      self.println(HELP);
      return;
    };

    let name = args.next().unwrap_or("default");
    // todo check if `name` is available to connect to

    let Ok(bridge) = TcpBridge::accepting_from(port) else {
      self.sys_msg(&format!("Cannot create Vlink on port '{port}'.\
Please make sure you have sufficient permissions and the port is not currently used by another process"));
      return;
    };
    self.opt_bridge = Some(bridge);
    self.sys_msg(
              &format!(
                "Successfully connected to the Vlink '{name}'.\nTraffic to your local port tcp:{port} will be routed to your peer"
              ),
            );
  }

  fn vlink_close(&mut self) {
    if self.opt_bridge.take().is_some() {
      let msg = Signal::VlinkCut;
      self.schedule_io(msg);
    } else {
      self.println("There was no Vlink to be closed")
    }
  }

  fn campfire(&mut self, args: SplitWhitespace) {
    let mut users: Vec<UserIdentifier> = args
      .map(|string| UserIdentifier {
        bs58: string.into(),
      })
      .collect();
    users.sort_by(|one, two| one.bs58.cmp(&two.bs58));

    let mut participants_list = users
      .iter()
      .fold(String::new(), |string, user| string + &user.bs58 + ",");
    participants_list.pop();
    let id = format!("campfire:campfire:{participants_list}");

    self
      .window()
      .emit("new-campfire", json!({ "id": id }))
      .expect("Failed to emit event");

    // send to all ":campfire-new"
    {
      let msg = Signal::Message(format!(":campfire-new {id}"));
      let networking = self.window().state::<Networking>();
      let mutex_guard = networking.chats.lock().expect("poisoned mutex");

      for user in users.iter() {
        let Some(tunnel) = mutex_guard.values().find(|&room| room.peer_id() == user) else{
            tracing::warn!("campfire cannot send msg: not connected to peer: {user:?}");
            continue;
          };

        let sender = tunnel.sender().clone();
        let msg = msg.clone();
        tokio::spawn(async move {
          if let Err(err) = sender.send(msg).await {
            tracing::warn!("error sending campfire-new message to a tunnel: {err}");
          }
        });
      }
    }

    let window = self.window().clone();
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
