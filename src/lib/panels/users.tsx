import { Component, For } from "solid-js";
import { User } from "../../types/user";

import "./users.css";

/**
 * Displays a list of users.
 */
const users: Component<{users: User[]}> = (props) => {
    return (
        <ol class="users">
            <For each={props.users}>{(user) =>
                <li>
                    <h4 class="name">{user.name ?? "Noname"}</h4>
                    <p class="id">{user.key}</p>
                </li>
            }</For>
        </ol>
    );
};

export default users;