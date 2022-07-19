use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EmberryMessage {
    Close()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RhizomeMessage {
    Error(String)
}
