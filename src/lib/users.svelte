<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { UserStatus, storeUser, type User, Leaf } from "./user";
  import { invoke } from "@tauri-apps/api/tauri";

  export let users: User[];

  onMount(() => {
    invoke("get_usrs", { limit: -1, offset: 0 }).then((b_users: any) => {
      b_users.forEach((user: any) => {
        let s_user = {
          key: user.identifier.bs58,
          name: user.info.username,
          status: UserStatus.Disconnected,
        };
        users.push(s_user);
      });
      // update the rendering
      users = users;

      //FIXME keep using store user while we dont have a better for status
      if (
        sessionStorage.getItem("has-mounted") === null ||
        sessionStorage.getItem("has-mounted") !== "true"
      ) {
        for (let i = 0; i < users.length; i++) {
          storeUser(users[i]);
        }
        sessionStorage.setItem("has-mounted", "true");
      }
    });

    listen("wants-room", (e: any) => {
      const usrkey = e.payload;

      let s_user = {
        key: usrkey,
        name: "Change wants-room to send IdentifiedUserInfo",
        status: UserStatus.Pending,
      };

      const userIndex = users.findIndex((u) => u.key === usrkey);
      // If this user is new then just push them into the array.
      if (userIndex === -1) {
        users.push(s_user);
        users = users; // Update the rendering
      }
      // If this user is already present then update their data.
      else users[userIndex] = s_user;
      //FIXME keep using store user while we dont have a better for status
      storeUser({ key: usrkey, status: UserStatus.Pending });
    });

    listen("new-room", (e: any) => {
      const usrkey = e.payload.peer_id;
      let s_user = {
        key: usrkey,
        name: "Change new-room to send IdentifiedUserInfo",
        status: UserStatus.Pending,
      };

      const userIndex = users.findIndex((u) => u.key === usrkey);
      if (userIndex === -1)
        console.error("new-room event but the user was not found in users");
      // If this user is already present then update their data.
      else users[userIndex] = s_user; // index assignment updated the rendering

      //FIXME keep using store user while we dont have a better for status
      storeUser({
        key: e.payload.peer_id,
        status: UserStatus.Connected,
      });
    });
  });
</script>

<div class="tools" />

<ol class="users">
  {#each users as user}
    <Leaf {user} />
  {/each}
</ol>

<style lang="scss">
  .tools {
    width: 100%;
    height: 60px;
    min-height: 60px;

    margin: 0 0 12px 0;
    background-color: #ffffff08;
    border-radius: 10px;
  }

  .users {
    width: 100%;
    height: 100%;

    margin: 0;
    padding: 8px 0 0 0;

    list-style: none;

    background-color: #2c2c2c;
    border-radius: 12px;
  }
</style>
