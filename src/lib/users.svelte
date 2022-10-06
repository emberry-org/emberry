<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import Leaf from "./user.leaf.svelte";
  import type { User } from "./user";
  import { UserStatus } from "./user.status";
  import { onMount } from "svelte";

  export let users: User[];

  onMount(() => {
    listen('wants-room', (e: any) => {
      const usrkey = e.payload.key;

      users.push({ key: usrkey, status: UserStatus.Pending });
      users = [...users];
    });

    // TODO: get the user that we've connected too
    listen("new-room", (_: any) => {
      users.push({ key: "unknown", status: UserStatus.Connected });
      users = [...users];
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

