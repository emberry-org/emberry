/**
 * Enum representing the status of a user on the network.
 */
export enum UserStatus {
  /** This user has requested a connection and is waiting for us to accept it */
  Pending,
  /** We are waiting for this user to accept our connection request */
  Awaiting,
  /** This user is disconnected from rhizome */
  Offline,
  /** This user is connected to rhizome */
  Online,
  /** This user is connected to us */
  Connected,
  /** We've connected to this user in the past but are now disconnected */
  Disconnected,
}

