<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Leaf from "./user.leaf.svelte";
  import type { User } from "./user";
  import { UserStatus } from "./user.status";
  import { onMount } from "svelte";
  import { storeUser } from "./user.store";
  import { onItem } from "./store";

  export let users: User[];

  onMount(() => {
    // Subscribe to the on item update event for users.
    users = JSON.parse(onItem(localStorage, (val) => {
      users = JSON.parse(val ?? "[]");
    }, "users") ?? "[]");

    listen('wants-room', (e: any) => {
      const usrkey = e.payload.key;

      users = storeUser({ key: usrkey, status: UserStatus.Pending });
    });

    listen("new-room", (e: any) => {
      // todo : retrieve the usrkey from the event.payload once that has been added.
      users = storeUser({ key: e.payload.peer_id, status: UserStatus.Connected });
    });
  });

</script>


<ol class="users">
  {#each users as user}
  
  <Leaf {user} />

  {/each}
</ol>


<style lang="scss">

.users {
  width: 100%;
  height: 100%;

  margin: 0;
  padding: 8px 0 0 0;

  list-style: none;
}

</style>

