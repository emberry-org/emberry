import { Component, createSignal, Index, lazy, onMount } from "solid-js";
import { useNavigate, useParams } from '@solidjs/router';
import { getItem, onItem } from "../lib/storage";
const Inputbox = lazy(() => import("../lib/inputbox"));

import './room.css';

interface Msg {
    origin: string,
    content: string
}

const Room: Component = () => {
    // Get the IDs from the route parameters:
    const { id } = useParams<{ id: string }>();
    const room_id = id.split(':')[1];
    const peer_id = id.split(':')[0];

    let messagesEl: HTMLDivElement;

    onMount(() => {
        // Scroll to bottom.
        messagesEl.scrollTop = messagesEl.scrollHeight;
    });

    const [chat, setChat] = createSignal<Msg[]>(JSON.parse(getItem(sessionStorage, `messages-${room_id}`)));

    onItem(`messages-${room_id}`, (messages) => {
        setChat(messages);

        // Check if the user has scrolled all the way to the bottom.
        if (messagesEl.scrollTop < messagesEl.scrollHeight - messagesEl.clientHeight - 100) return;

        // Scroll down.
        messagesEl.scrollTop = messagesEl.scrollHeight;
    });

    const navigate = useNavigate();

    return <div class="room">
        <div class="header">
            <h2>Room</h2>
            <p>Room id: {room_id}</p>
            <p>Peer id: {peer_id}</p>
            <button onMouseDown={() => navigate("/", { replace: true })}>Return to Home</button>
        </div>
        
        <div class="messages" ref={messagesEl}>
            <Index each={chat()}>{(msg) =>
                <li>
                {msg().origin}: {msg().content}
                </li>
            }</Index>
        </div>
        <Inputbox room_id={room_id} />
    </div>;
};

export default Room;
