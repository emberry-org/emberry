import type { UserStatus } from "lib/user";

/**
 * User information payload.
 */
export interface UserInfo {
  id: string;
  name: string;
  status?: UserStatus;
}