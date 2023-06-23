import { invoke } from '@tauri-apps/api/tauri';
import { listen } from "@tauri-apps/api/event";
import { getItem, updateItem } from './lib/storage';
import { Room, RoomState } from './types/room';

/** Event for when a new room is created. */
interface NewRoomEvent {
    payload: {
        room_id: string,
        peer_id: string
    }
}

/** Event for when a new campfire is created. */
interface NewCampfireEvent {
    payload: {
        id: string,
    }
}

/** Event for when a new room request is received. */
interface IdentifiedUserInfo {
    identifier: {
        bs58: string,
    },
    info: {
        username: string,
        relation: number
    }
}

/**
 * Initialize all the Tauri listeners and connections.
 */
const initTauri = () => {
    // Connect to the Rhizome server:
    invoke('connect').then(() => {
        console.log("rhizome connection established!");
    }).catch((e) => {
        if (e === "Already connected to the server") {
            console.log("rhizome already connected!");
        } else {
            console.error(e);
        }
    }); // TAURI

    // Listen for new rooms:
    listen("new-room", (e: NewRoomEvent) => {
        const room_id = e.payload.room_id;
        const peer_id = e.payload.peer_id;

        // Update the rooms array:
        updateItem(sessionStorage, "rooms", (rooms: any) => {
            if (!rooms) rooms = {};
            // Update the new room.
            rooms[peer_id] = <Room>{
                ...rooms[peer_id],
                roomId: room_id,
                state: RoomState.Online
            };
            return rooms;
        });

        // TODO: this should be re established if the app is refreshed.
        // Listen for incoming message on this room:
        listen(`user_msg_${room_id}`, (e: any) => {
            const content: string = e.payload;

            const room: Room = JSON.parse(getItem(sessionStorage, "rooms"))[peer_id];
            updateItem(sessionStorage, `messages-${room_id}`, (chat: { origin: string, content: string }[]) => {
                if (chat == null) chat = [];
                chat.push({
                    origin: room.name,
                    content: content,
                });
                return chat;
            });
        }); // TAURI

        // Listen for incoming system message on this room:
        listen(`sys_msg_${room_id}`, (e: any) => {
            const content: string = e.payload;

            updateItem(sessionStorage, `messages-${room_id}`, (chat: { origin: string, content: string }[]) => {
                if (chat == null) chat = [];
                chat.push({
                    origin: "[SYSTEM]",
                    content: content,
                });
                return chat;
            });
        }); // TAURI

        console.log(`p2p new room : ${peer_id}:${room_id}`);
    }); // TAURI

    // Listen for campfire rooms:
    listen("new-campfire", (e: NewCampfireEvent) => {
        const room_id = e.payload.id;
        const peer_id = e.payload.id;

        // Update the rooms array:
        updateItem(sessionStorage, "rooms", (rooms: any) => {
            if (!rooms) rooms = {};
            // Update the new room.
            rooms[peer_id] = <Room>{
                ...rooms[peer_id],
                roomId: room_id,
                state: RoomState.Online
            };
            return rooms;
        });

        // TODO: this should be re established if the app is refreshed.
        // Listen for incoming message on this room:
        listen(`user_msg_${room_id}`, (e: any) => {
            const content: string = e.payload.msg;
            const sender = e.payload.sender as IdentifiedUserInfo;

            const room: Room = JSON.parse(getItem(sessionStorage, "rooms"))[peer_id];
            updateItem(sessionStorage, `messages-${room_id}`, (chat: { origin: string, content: string }[]) => {
                if (chat == null) chat = [];
                chat.push({
                    origin: sender.info.username,
                    content: content,
                });
                return chat;
            });
        }); // TAURI

        console.log(`p2p new room : ${peer_id}:${room_id}`);
    }); // TAURI
    // Listen for new room requests:
    listen("wants-room", (e: any) => {
        const room = e.payload as IdentifiedUserInfo;

        // Update the rooms array:
        updateItem(sessionStorage, "rooms", (rooms: any) => {
            if (!rooms) rooms = {};
            // Add the new room.
            rooms[room.identifier.bs58] = <Room>{
                peerId: room.identifier.bs58,
                name: room.info.username,
                state: RoomState.Pending
            };
            return rooms;
        });

        // Set the room request timeout at 10s
        setTimeout(() => {
            updateItem(sessionStorage, "rooms", (rooms: any) => {
                if (!rooms) rooms = {};

                // Do nothing if the room was created succesfully.
                if (rooms[room.identifier.bs58] != undefined &&
                    rooms[room.identifier.bs58].state == RoomState.Online
                ) {
                    return rooms;
                }

                // Remove the room since it timedout:
                rooms[room.identifier.bs58] = undefined;
                return rooms;
            });
        }, 10000);

        console.log(`p2p room requested : ${room.identifier.bs58}`);
    }); // TAURI

    // Get the list of known users.
    // invoke("get_usrs", { limit: -1, offset: 0 }).then((res: any[]) => {
    //     const users = res.map(u => <User>{
    //         key: u.identifier.bs58,
    //         name: u.info.username
    //     });
    //     setItem(sessionStorage, "known_users", JSON.stringify(users));
    // }); // TAURI

    // TODO: listen("error", (e: { payload: error }))

    // TODO: listen("punching", (e: { payload: ident }))

    // TODO: listen("new-user", (e: { payload: bs58 }))
    // TODO: listen(`usr_name_${id}`, (e: { payload: username }))

    // setItem(sessionStorage, "rooms", JSON.stringify({
    //     "MYROOMID": {
    //         peerId: "MYROOMID",
    //         roomId: "MYROOMID",
    //         name: "my_room",
    //         state: RoomState.Online
    //     },
    //     "MYOTHERROOMID": {
    //         id: "MYOTHERROOMID",
    //         name: "other_room",
    //         state: RoomState.Pending
    //     }
    // }));
};

export default initTauri;
