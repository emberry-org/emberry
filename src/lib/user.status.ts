/**
 * Enum representing the status of a user on the network.
 */
export enum UserStatus {
  /** We are waiting for this user to accept our connection request */
  Pending,
  /** This user is disconnected from rhizome */
  Offline,
  /** This user is connected to rhizome */
  Online,
  /** This user is connected to us */
  Connected,
}

