<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { UserStatus, type User, Leaf } from "./user";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onStatusChange, setItem } from "./store";

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
        listen(`usr_name_${s_user.key}`, (e: any) => {
          console.log("event for ", s_user.key);
          const userIndex = users.findIndex((u) => u.key === s_user.key);
          if (userIndex === -1)
          {
            console.error(`username of non existing user "${s_user.key}" changed`);
            return;
          }
          users[userIndex].name = e.payload;
        });
      });
      // update the rendering
      users = users;
    });

    onStatusChange((key: string | null, value: UserStatus) => {
      const userIndex = users.findIndex((u) => u.key === key);
      if (userIndex === -1)
      {
        console.error(`Status of non existing user "${key}" changed`);
        return;
      }
      users[userIndex].status = value;
    });

    listen("wants-room", (e: any) => {
      const usrkey = e.payload.identifier.bs58;

      let s_user = {
        key: usrkey,
        name: e.payload.info.username,
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
      setItem(usrkey, JSON.stringify(UserStatus.Pending));
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
