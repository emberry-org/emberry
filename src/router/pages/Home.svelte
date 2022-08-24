<script lang="ts">
  import Icon from "@icon";
  import { insertTab, navigateTo } from "@store";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from '@tauri-apps/api/event';
  import { fade } from 'svelte/transition';
  import { onMount } from "svelte";

  let chatId: string;
  let error: string = '';
  let connecting: boolean = false;

  onMount(() => {
    listen("new-room", (event) => {
      navigateTo('/chat/' + event.payload);
    });
  });

  function attemptConnect() {

    navigateTo('/chat/1234');
    return;

    connecting = true;
    invoke('hole_punch', { peerKey: chatId }).then((id: string) => {
      // Create a new tab once the chat has been created.
      insertTab({ icon: 'chat', title: id.substring(0, 6), path: '/chat/' + id, keep_open: true });
      // Navigate to the chat tab.
      navigateTo('/chat/' + id);
    }).catch((err) => {
      error = err;
    });
  }
</script>

<div class="home" transition:fade={{ duration: 200 }}>

  <div class="left-panel">

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
