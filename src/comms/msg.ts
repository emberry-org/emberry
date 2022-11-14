import { emit, listen } from "@tauri-apps/api/event";
import type { MessageEvent } from "./msg.event";

/**
 * Send a message in a room.
 * @param id The id of the room.
 * @param payload The payload of the message.
 */
 export function sendMessage(id: string, payload: any) {
  emit(`send_message_${id}`, payload);
}

/**
 * Send a username update in a room.
 * @param id The id of the room.
 * @param name The name of the local user.
 */
export function sendUsername(id: string, name: any) {
  emit(`send_message_${id}`, { Username: name });
}

/**
 * Add a listener to the message recieved event.
 * `message_recieved_<id>`
 * @param id The id of the room.
 * @param cb A callback for whenever the event is fired.
 */
export function onMessage(id: string, cb: (e: MessageEvent) => void) {
  listen(`message_recieved_${id}`, (e: any) => {
    /* Rust to Typescript types shenanigans */
    const type: string = Object.keys(e.payload.message)[0];

    cb({id,
      msg: {
        content: e.payload.message[type],
        type,
      }
    });
  });
}