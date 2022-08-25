<script lang="ts">
  import { navigateTo } from "@store";
  import { listen } from '@tauri-apps/api/event';
  import { fade } from 'svelte/transition';
  import { onMount } from "svelte";
  import UserList from "@lib/generic/users/UserList.svelte";

  onMount(() => {
    listen("new-room", (event) => {
      navigateTo('/chat/' + event.payload);
    });
  });
</script>

<div class="home" transition:fade={{ duration: 200 }}>

  <div class="left-panel">
    <UserList />
  </div>

  <div class="center-panel">

  </div>

  <div class="right-panel">

  </div>

</div>

<style lang="scss">
  .home {
    width: 100%;
    height: 100%;

    display: flex;

    .left-panel {
      width: 270px;
      height: 100%;

      margin-right: 24px;

      background-color: var(--mg);
      border-bottom-left-radius: 12px;
      border-bottom-right-radius: 12px;
    }

    .center-panel {
      flex-grow: 1;
      height: 100%;

      background-color: var(--fg);
      border-bottom-left-radius: 12px;
      border-bottom-right-radius: 12px;
    }

    .right-panel {
      width: 270px;
      height: 100%;

      margin-left: 24px;

      background-color: var(--mg);
      border-bottom-left-radius: 12px;
    }
  }
</style>
