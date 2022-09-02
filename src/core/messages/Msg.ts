import type { Remoji } from "./Remoji";

/**
 * Base template for a chat message.
 */
export interface Msg {
  /** The sender of this message. */
  sender: String,
  /** The content of this message. */
  content: String,
  /** The time at which this message was send. */
  time: String,
  
  /** Reaction emojis. ``(undefined if there are no emoji reactions)`` */
  remojis?: Array<Remoji> | undefined,

  /** The target message of this message. ``(undefined if not a reaction)`` */
  target?: Msg | undefined,

  /** Whether this message is a ping for the local user. ``(undefined if false)`` */
  ping?: Boolean | undefined,
}

/**
 * Extension for thread messages.
 */
export interface ThreadMsg extends Msg {
  /** The messages send in this thread. */
  messages: Array<Msg>,

  /* NOTE: The title of the thread is stored within the ``Msg -> content`` */
}