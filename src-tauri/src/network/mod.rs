use std::collections::HashMap;
use std::sync::Mutex;

use smoke::messages::RoomId;
use smoke::User;
use tauri::EventHandler;

use tokio::sync::oneshot;

pub mod ctrl_chnl;
mod holepunch;
mod p2p_tunl;

type ConnectionMap = HashMap<RoomId, Connection>;
pub struct Connection {
  pub send_handle: EventHandler,
  pub recv_handle: oneshot::Sender<()>,
}

pub enum RRState {
  Pending,
  Agreement,
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
