import type { UserStatus } from ".";

/**
 * Interface representing a user on the network.
 */
export interface User {
  key: string;
  name?: string;
  /** Base64 encoded */
  avatar?: string;
  status: UserStatus;
  activity?: string;
}

