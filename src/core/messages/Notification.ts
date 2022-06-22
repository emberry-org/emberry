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
  UnreadPing,
}

/**
 * Get an icon for the given notification type.
 */
export function getNotificationIcon(type: NotificationType): string {
  switch (type) {
    case NotificationType.ConnectionRequest: return "connect";
    case NotificationType.FriendRequest: return "sprout";
    case NotificationType.UnreadMessage: return "chat";
    case NotificationType.UnreadPing: return "bell";
    default: return "";
  }
}

/**
 * Get a style for the given notification type.
 */
 export function getNotificationStyle(type: NotificationType): string {
  switch (type) {
    case NotificationType.ConnectionRequest: return "background-color: #daaa3f; box-shadow: 0 0 0 2px #daaa3f;";
    case NotificationType.FriendRequest: return "background-color: #479b4a; box-shadow: 0 0 0 2px #222;";
    case NotificationType.UnreadMessage: return "background-color: #539bf5; box-shadow: 0 0 0 2px #222;";
    case NotificationType.UnreadPing: return "background-color: #8e53f5; box-shadow: 0 0 0 2px #8e53f5;";
    default: return "";
  }
}