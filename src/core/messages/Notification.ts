import type { PeerStatus } from "@core/PeerStatus"

export interface Notification {
  type: NotificationType,
  user: {
    id: String,
    status: PeerStatus
  }
}

export enum NotificationType {
  ConnectionRequest,
  FriendRequest,
  UnreadMessage,
  IdleChat,
  UnreadPing,
}

/**
 * Get an icon for the given notification type.
 */
export function getNotificationIcon(type: NotificationType): string {
  switch (type) {
    case NotificationType.ConnectionRequest: return "notifications/connect";
    case NotificationType.FriendRequest: return "notifications/friend";
    case NotificationType.UnreadMessage: return "notifications/msg";
    case NotificationType.IdleChat: return "notifications/chat";
    case NotificationType.UnreadPing: return "notifications/ping";
    default: return "";
  }
}

/**
 * Get a style for the given notification type.
 */
 export function getNotificationStyle(type: NotificationType): string {
  switch (type) {
    case NotificationType.ConnectionRequest: return "color: #63c466; background-color: #479b4a33; box-shadow: 0 0 0 2px #479b4a33;";
    case NotificationType.FriendRequest: return "color: #9c73d4; background-color: #66479b55; box-shadow: 0 0 0 2px #66479b55;";
    case NotificationType.UnreadMessage: return "color: #639ac4; background-color: #477b9b44; box-shadow: 0 0 0 2px #477b9b44;";
    case NotificationType.IdleChat: return "color: #777; background-color: #333; box-shadow: 0 0 0 2px #333;";
    case NotificationType.UnreadPing: return "color: #df4848; background-color: #ef585833; box-shadow: 0 0 0 2px #ef585833;";
    default: return "";
  }
}