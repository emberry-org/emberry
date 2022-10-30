use std::io;

use smoke::Signal;
use tauri::Window;

use crate::network::MessageRecievedPayload;

pub async fn handle_signal(signal: &Signal, spawn_window: &Window, event_name: &str) -> Result<(), io::Error> {
  /* Emit the message recieved event */
  spawn_window
    .emit(event_name, MessageRecievedPayload { message: signal })
    .expect("Failed to emit event");

  Ok(())
}
