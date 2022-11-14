import type { UserStatus } from "lib/user";

/**
 * User information payload.
 */
export interface UserPayload {
  id: string;
  name: string;
  status?: UserStatus;
}