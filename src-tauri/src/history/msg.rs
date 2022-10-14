use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Msg {
  /// Normal text message
  Text(Text),
  /// Inline reaction message
  InlineReaction(InlineReaction),
  // Image, Embeds, Invites, etc...
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// # Text
/// The default chat message.
/// These contain text and a sender.
///
/// # Format
/// ```
/// @sender : text message content
/// ```
pub struct Text {
  /** ID of the user who send this message */
  sender: u8,
  /** Content of the message */
  content: String,
  /** Time at which the message was send */
  // time: u64, // This should be a DateTime<UTC>

  /** Reaction emojis */
  remojis: Vec<Remoji>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
/// # Inline Reactions
/// A reaction to another message in the same chat.
/// These reactions are inserted at the bottom of the chat.
/// Just like normal text messages except the refer to another message.
///
/// # Format
/// ```
///            ↱ @sender — content
/// @sender : text message content
/// ```
pub struct InlineReaction {
  /** ID of the user who send this message */
  sender: u8,
  /** Content of the message */
  content: String,
  /** Time at which the message was send */
  // time: u64, // This should be a DateTime<UTC>

  /** Reaction emojis */
  remojis: Vec<Remoji>,
  /** The message this reaction is targeting */
  target: Box<Msg>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
/// # Reaction Emojis
/// All messages can be reacted too using emojis.
///
/// # Format
/// ```
/// @sender : text message content
///            😃 4  😵 6
/// ```
pub struct Remoji {
  charCode: u8,
  quantity: u16,
}
