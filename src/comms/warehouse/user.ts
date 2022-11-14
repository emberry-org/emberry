import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { UserStatus, type User } from "lib/user";
import type { UserPayload, UserUpdatedEvent } from "./user.event";

/**
 * Set the local username.
 * @param name The new username.
 */
export function setUsername(name: string) {
  invoke("update_username", { name });
}

/**
 * Get the current user list.
 * @returns All users in the user list.
 */
export async function getUserList(): Promise<User[]> {
  const users = await invoke("get_usrs", { limit: -1, offset: 0 }) as any[];
  return users.map(u => <User>{
    key: u.identifier.bs58,
    name: u.info.username,
    status: UserStatus.Disconnected,
  });
}

/**
 * Get the user information of the local user.
 * @returns The information of the local user.
 */
export async function getLocalUserInfo(): Promise<UserPayload> {
  const event = await invoke("get_local") as any;

  if (event === null) {
    return {
      id: "unknown",
      name: "[no_user_pem]"
    };
  }

  return { 
    id: event.identifier.bs58,
    name: event.info.username
  };
}

/**
 * Get the user information of a user.
 * @param id The id of the user.
 * @returns The information of the user.
 */
export async function getUserInfo(id: string): Promise<UserPayload> {
  const event = await invoke("get_usr_info", { bs58cert: id }) as any;
  return { id,
    name: event.username
  };
}

/**
 * Add a listener to the user information event.
 * @param id The id of the user.
 * @param cb A callback for whenever the event is fired.
 */
export async function onUserInfo(id: string, cb: (e: UserUpdatedEvent) => void) {
  listen(`usr_name_${id}`, (name: any) => {
    cb({
      name: name.payload
    });
  });
}