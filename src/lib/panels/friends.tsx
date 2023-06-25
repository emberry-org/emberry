import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { User } from "../../types/user";
import UsersList from "./users";

import "./friends.css";

const friends: Component = () => {
    const [users, setUsers] = createSignal<User[]>([]);

    {
        invoke("get_usrs", { limit: 100, offset: 0 }).then((e: any[]) => {
            setUsers(e.map(user => ({
                key: user.identifier.bs58,
                name: user.info.username.trim().length > 0 ? user.info.username : undefined
            })));
        });
    }
  
    return (
        <section class="panel friends">
            <UsersList users={users()} />
        </section>
    );
};

export default friends;