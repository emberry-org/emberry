import { Component, createSignal, Index, lazy, onMount, Show } from "solid-js";
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
    const params = id.split('~');
    const room_id = params[1];
    const peer_id = params[0];

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
            <h2><button onMouseDown={() => navigate("/", { replace: true })}>Return to Home</button> Room</h2>
            <p>Room id: {room_id}</p>
            <p>Peer id: {peer_id}</p>
        </div>

        <div class="messages" ref={messagesEl}>
            <li class="begin">The beginning of a new chat room.</li>
            <Index each={chat()}>{(msg, i) =>
                <li>
                    <Show
                        when={i <= 0 || chat()[i - 1].origin !== msg().origin}
                    >
                        <h3>{msg().origin}</h3>
                    </Show>
                    <div>{msg().content}</div>
                </li>
            }</Index>
        </div>
        <Inputbox room_id={room_id} />
    </div>;
};

export default Room;
