use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use smoke::messages::RoomId;
use smoke::User;
use tauri::EventHandler;

use tokio::sync::oneshot;

pub use user_identification::UserIdentification;

pub mod ctrl_chnl;
mod p2p_tunl;
mod resolver;
mod rhizome;
mod tls_kcp;
mod user_identification;

type ConnectionMap = HashMap<RoomId, Connection>;
pub struct Connection {
  pub send_handle: EventHandler,
  pub recv_handle: oneshot::Sender<()>,
}

pub enum RRState {
  /// In this state we (local client) have sent a room request to the other person
  Requested(Instant),
  /// In this state both (locan and peer) have agreed to form a p2p connection
  /// "bool" details whether the local user has initiated the room request (logging)
  Accepted,
}

pub struct Networking {
  pub chats: Mutex<ConnectionMap>,
  pub pending: Mutex<HashMap<User, RRState>>,
}

#[tauri::command]
pub fn chat_exists(state: tauri::State<'_, Networking>, id: RoomId) -> bool {
  // Check if the store contains the key for this chat.
  match state.chats.lock() {
    Ok(chats) => chats.contains_key(&id),
    Err(_) => false,
  }
}
