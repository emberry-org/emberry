import type { NotificationType } from "./messages/Notification";
import type { PeerStatus } from "./PeerStatus";

/**
 * Time for a nice drink at the pub :D
 */
export default interface Drink {
  type: NotificationType;
  user: { 
    id: string;
    status: PeerStatus;
  };
}