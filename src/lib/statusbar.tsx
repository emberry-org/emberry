import Telescope from '@ico/telescope.svg?component-solid';
import { onItem } from './storage';
import { createSignal, For } from 'solid-js';

import './statusbar.css';
import { Room, RoomState } from '../types/room';
import { invoke } from '@tauri-apps/api/tauri';
import { useNavigate } from '@solidjs/router';

export default () => {
    const [rooms, setRooms] = createSignal<Room[]>([]);

    // Listen for updates to the rooms:
    onItem("rooms", (rooms) => {
        const keys = Object.keys(rooms);
        let new_rooms: Room[] = [];

        for (let i = 0; i < keys.length; i++)
            new_rooms.push(rooms[keys[i]]);

        // Update the rooms.
        setRooms(new_rooms);
    });
    
    const navigate = useNavigate();

    // Called when a room is clicked:
    const onClick = (room: Room) => {
        switch (room.state) {
            case RoomState.Pending:
                // TEMP: Just auto accept if clicked.
                invoke("accept_room", {
                    bs58cert: room.peerId,
                    accepted: true,
                }); // TAURI
                break;

            case RoomState.Online:
                if (room.roomId) {
                    navigate(`/room/${room.peerId}:${room.roomId}`, { replace: true });
                }
                break;
        
            default:
                break;
        }
        console.log("clicked room: " + room.name);
    };

    return (
        <div class="statusbar">
            {/* <Telescope width="24px" /> */}
            <For each={rooms()}>
                {(room, i) =>
                    <button onMouseDown={() => onClick(room)}>
                        {i() + 1}: {room.name} <sup>{room.state}</sup>
                    </button>
                }
            </For>
        </div>
    );
}
