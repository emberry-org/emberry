import { getItem, onItem } from './storage';
import { createSignal, For, Show } from 'solid-js';
import { Room, RoomState } from '../types/room';
import { invoke } from '@tauri-apps/api/tauri';
import { useNavigate } from '@solidjs/router';

import Goto from '@ico/uparrow.svg?component-solid';
import Received from '@ico/downarrow.svg?component-solid';
import './statusbar.css';

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

    {
        const rooms = JSON.parse(getItem(sessionStorage, "rooms"));
        const keys = Object.keys(rooms);
        let new_rooms: Room[] = [];

        for (let i = 0; i < keys.length; i++)
            new_rooms.push(rooms[keys[i]]);

        // Update the rooms.
        setRooms(new_rooms);
    }
    
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
    };

    return (
        <div class="statusbar">
            <For each={rooms()}>
                {(room) =>
                    <button onMouseDown={() => onClick(room)} disabled={room.state == RoomState.Awaiting}>
                        {room.name} <sup>{room.state}</sup>
                        
                        {/* Render the recieved or goto icon */}
                        <Show
                            when={room.state != RoomState.Pending}
                            fallback={<Received width="20px" style="margin: 0 -4px 0 8px" />}
                        >
                            <Goto width="20px" style={room.state == RoomState.Online ? "margin: 0 -4px 0 8px; color: #ddd" : "margin: 0 -4px 0 8px; color: var(--bg-500)"} />
                        </Show>
                    </button>
                }
            </For>
        </div>
    );
}
