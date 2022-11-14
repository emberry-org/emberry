/**
 * User information payload.
 */
export interface UserPayload {
  id: string;
  name: string;
}

/**
 * Event information for the user update event.
 */
export interface UserUpdatedEvent {
  name?: string;
}