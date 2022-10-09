import { setItem } from "./store";
import type { UserStatus } from "./user.status";

/**
 * Interface representing a user stored in our local storage.
 */
export interface StoredUser {
  key: string;
  name?: string;
  avatar?: string;
  status: UserStatus;
}

/**
 * Store a new user / update an already stored user within the local storage.
 * @param user The user data.
 * @returns The updated users array.
 */
export function storeUser(user: StoredUser): StoredUser[] {
  const stored: StoredUser[] = JSON.parse(localStorage.getItem("users") ?? "[]");
  const userIndex = stored.findIndex(u => u.key === user.key);
  // If this user is new then just push them into the array.
  if (userIndex === -1) stored.push(user);
  // If this user is already stored then update their data.
  else stored[userIndex] = user;
  setItem(localStorage, "users", JSON.stringify(stored));
  return stored;
}