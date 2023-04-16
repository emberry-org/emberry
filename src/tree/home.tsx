import { useNavigate } from "@solidjs/router";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from '@tauri-apps/api/clipboard';
import { Component, createSignal } from "solid-js";
import { setItem, updateItem } from "../lib/storage";
import { RoomState } from "../types/room";

import Clipboard from '@ico/clipboard.svg?component-solid';
import Link from '@ico/link.svg?component-solid';
import './home.css';

interface UserInfo {
    identifier: {
        bs58: string
    },
    info: {
        username: string,
        relation: number,
    }
}

const home: Component = () => {
    let input: HTMLInputElement;

    /** Send a room request */
    const sendRoomRequest = () => {
        const cert = input.value;
        invoke("request_room", { bs58cert: cert }); // TAURI
        setItem(sessionStorage, cert, "awaiting");

        // Update the rooms array:
        updateItem(sessionStorage, "rooms", (rooms: any) => {
            if (!rooms) rooms = {};
            // Add the new room.
            rooms[cert] = {
                peerId: cert,
                name: "Requested Room",
                state: RoomState.Awaiting
            };
            return rooms;
        });

        // Set the room request timeout at 10s
        setTimeout(() => {
            updateItem(sessionStorage, "rooms", (rooms: any) => {
                if (!rooms) rooms = {};

                // Do nothing if the room was created succesfully.
                if (rooms[cert] != undefined &&
                    rooms[cert].state == RoomState.Online
                ) {
                    return rooms;
                }

                // Remove the room since it timedout:
                rooms[cert] = undefined;
                return rooms;
            });
        }, 10000);

        console.log(`send room request to: ${cert}`);
        input.value = "";
    };

    // Get the local user info:
    const [user, setUser] = createSignal<UserInfo>(undefined);
    invoke("get_local").then((e: any) => {
        setUser(() => {
            return e as UserInfo;
        });
    }); // TAURI

    const username = () => user() ? user().info.username : "unknown";
    const key = () => user() ? user().identifier.bs58 : "unknown";

    /** Copy the local user key to the clipboard */
    const copyKey = () => {
        writeText(key()); // TAURI
    };

    const navigate = useNavigate();

    return (
        <div>
            <h3 class="title">
                <span style="color: #666">Welcome back,</span> {username} 
                <div style="display: flex">
                    <button class="btn" onMouseDown={copyKey}>
                        <Clipboard class="ico" width="16px" height="16px" />
                        Copy key
                    </button>
                    <button class="btn" onMouseDown={() => navigate('/room/peer_id:room_id', { replace: true })}>
                        <Link class="ico" width="16px" height="16px" />
                        Test Room
                    </button>
                </div>
            </h3>
            
            <h3 class="sub-title">Request Room:</h3>
            <input ref={input} type="text" placeholder="Enter peer key..." name="Request Room" />
            <button class="btn-submit" onClick={sendRoomRequest} type="submit">Send Request</button>
        </div>
    );
};

export default home;
