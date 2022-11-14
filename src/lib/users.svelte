<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { UserStatus, type User, Leaf } from "./user";
  import { onStatusChange, setItem } from "./store";
  import { getUserList, onNewUser, onUserInfo } from "comms/warehouse";
  import { merge } from "utils/object";

  export let users: User[];

  onMount(() => {
    /* Load the user list */
    getUserList().then((list) => {
      users = list;

      /* Listen for user info updates from all users within the list */
      users.forEach(user => {
        onUserInfo(user.key, (diff) => {
          const i = users.findIndex((u) => u.key === user.key);
          if (i < 0) { console.error(`username updated of non-existing user (${ user.key })`); return; }

          users[i] = merge(users[i], <User>diff);
        });
      });
    });

    onNewUser((user: User) => {
      // If this user is new then just push them into the array.
      users.push(user);

      onUserInfo(user.key, (diff) => {
        const i = users.findIndex((u) => u.key === user.key);
        if (i < 0) { console.error(`username updated of non-existing user (${ user.key })`); return; }

        users[i] = merge(users[i], <User>diff);
      });

      users = users; // Update the rendering
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
