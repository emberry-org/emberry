use serde::{Deserialize, Serialize};
use smoke::messages::EmbMessage;

#[derive(Serialize, Deserialize)]
pub enum EmberryMessage {
    Close(),
    Direct(EmbMessage)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RhizomeMessage {
    Error(String)
}
