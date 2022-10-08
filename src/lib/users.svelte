<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Leaf from "./user.leaf.svelte";
  import type { User } from "./user";
  import { UserStatus } from "./user.status";
  import { onMount } from "svelte";
  import { storeUser } from "./user.store";

  export let users: User[];

  onMount(() => {
    listen('wants-room', (e: any) => {
      const usrkey = e.payload.key;

      users = storeUser({ key: usrkey, status: UserStatus.Pending });
    });

    listen("new-room", (_: any) => {
      // todo : retrieve the usrkey from the event.payload once that has been added.
      users = storeUser({ key: "unknown", status: UserStatus.Connected });
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
  padding: 0;

  list-style: none;
}

</style>

