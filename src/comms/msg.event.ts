/**
 * Message event.
 */
 export interface MessageEvent {
  id: string;
  msg: Message;
}

/**
 * Message payload.
 */
export interface Message {
  type: string;
  content: any;
}