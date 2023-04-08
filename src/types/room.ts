export enum RoomState {
    /** This room has been requested a connection and is waiting for us to accept it */
    Pending = "Pending",
    /** We are waiting for the other user to accept our room request */
    Awaiting = "Awaiting",
    /** This room is connected */
    Online = "Online",
}

export interface Room {
    /** ID of the other peer */
    peerId: string,
    /** ID of the room (is undefined on pending) */
    roomId?: string,
    /** Name of the room */
    name: string,
    /** Current state of the room */
    state: RoomState
}
