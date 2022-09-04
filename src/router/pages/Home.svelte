<script lang="ts">
  import { navigateTo } from "@store";
  import { listen } from '@tauri-apps/api/event';
  import { fade } from 'svelte/transition';
  import { onMount } from "svelte";
  import UserList from "@lib/generic/users/UserList.svelte";
  import Icon from "@icon";
  import Chat from "@lib/chat/Chat.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
import SidePanel from "@lib/generic/SidePanel.svelte";

  onMount(() => {
    listen("new-room", (event) => {
      navigateTo('/chat/' + event.payload);
    });
    invoke('save_history');
  });
</script>

<div class="home" transition:fade={{ duration: 200 }}>

  <SidePanel>
    <div class="left-panel">
      <div class="toolbar">
        <button class="btn add-friend">
          <Icon name="add-friend" size="24px" />
        </button>
        <input class="input search-bar" placeholder="Lookup friends..." />
        <button class="btn search">
          <Icon name="search" size="18px" />
        </button>
      </div>
      <UserList />
    </div>
  </SidePanel>

  <div class="center-panel">
    <Chat />
  </div>

  <SidePanel position="right">
    <div class="right-panel">

    </div>
  </SidePanel>

</div>

<style lang="scss">
  .home {
    width: 100%;
    height: 100%;

    display: flex;

    .left-panel {
      width: 100%;
      height: calc(100% - 8px);

      background-color: var(--mg);
      border-bottom-left-radius: 10px;
      border-bottom-right-radius: 10px;
      padding-top: 8px;

      .toolbar {
        width: calc(100% - 32px);
        height: 42px;

        padding: 0 16px;

        display: flex;
        align-items: center;

        .btn {
          width: 32px;
          min-width: 32px;
          height: 32px;
          padding: 0;

          display: flex;
          justify-content: center;
          align-items: center;

          &.add-friend {
            margin-right: 16px;
          }

          &.search {
            border-left: none;
            border-top-left-radius: 0;
            border-bottom-left-radius: 0;
          }
        }

        .search-bar {
          width: 100%;
          height: 30px;

          border-right: none;
          border-top-right-radius: 0;
          border-bottom-right-radius: 0;

          padding-left: 10px;
          font-size: 14px;

          background-color: var(--bg);
        }
      }
    }

    .center-panel {
      flex-grow: 1;
      height: 100%;

      background-color: var(--fg);
      border-bottom-left-radius: 10px;
      border-bottom-right-radius: 10px;
    }

    .right-panel {
      width: 100%;
      height: 100%;

      background-color: var(--mg);
      border-bottom-left-radius: 10px;
    }
  }
</style>
