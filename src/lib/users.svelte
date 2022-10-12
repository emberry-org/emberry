<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { UserStatus, storeUser, type User, Leaf } from "./user";
  import { onItem } from "./store";

  export let users: User[];

  onMount(() => {
    // Subscribe to the on item update event for users.
    users = JSON.parse(onItem(localStorage, (val) => {
      users = JSON.parse(val ?? "[]");
    }, "users") ?? "[]");

    if (sessionStorage.getItem("has-mounted") === null || sessionStorage.getItem("has-mounted") !== "true") {
      // Mark all users as disconnected on app load.
      for (let i = 0; i < users.length; i++) {
        users[i].status = UserStatus.Disconnected;
        storeUser(users[i]);
      }
      sessionStorage.setItem("has-mounted", "true");
    }

    listen('wants-room', (e: any) => {
      const usrkey = e.payload.key;

      users = storeUser({ key: usrkey, status: UserStatus.Pending });
    });

    listen("new-room", (e: any) => {
      users = storeUser({ key: e.payload.peer_id, status: UserStatus.Connected });
    });
  });

</script>


<div class="tools">

</div>

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

  background-color: #2C2C2C;
  border-radius: 12px;
}

</style>

