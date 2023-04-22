use std::str::SplitWhitespace;

use smoke::Signal;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::info;
use vlink::TcpBridge;

use crate::network::peer_tunnel::runtime::PeerTunnelRuntime;

const HELP: &str = r#"Available slash commands:
  `/help` or `/` - print this help
  `/vlink_open <port: u16> [<name: String>]` - open your local port <port> for a TCP Bridge connection
    <name> = `default`
  `/vlink_connect <port: u16> [<name: String>]` - connect to a TCP bridge opened by your peer
    <name> = `default`
  `/vlink_close` - close an existing TCP Bridge, when run after `/vlink_open` also terminates the peer end of the Bridge
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
      Some("/vlink_close") => self.vlink_connect(args),
      Some("/vlink_connect") => self.vlink_close(),
      None | Some(_) => self.println(HELP),
    }
  }

  fn println(&mut self, msg: &str);
  fn vlink_open(&mut self, args: SplitWhitespace);
  fn vlink_connect(&mut self, args: SplitWhitespace);
  fn vlink_close(&mut self);
}

pub enum Action {
  Capture,
  Pass,
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

    let name = args.next().unwrap_or("default");
    // todo check if `name` is available to connect to

    let Some(Ok(port)) = args.next().map(|port_str| port_str.parse::<u16>()) else {
      self.println("Invalid arguments for `/vlink_connect`");
      self.println(HELP);
      return;
    };

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
}
