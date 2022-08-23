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
  <!-- <div class="toolbar" /> -->

  <div class="card">

    <Icon name="app/logo" size="40vh" />
    
    {#if error !== ''}
      <div class="error">{ error }</div>
    {/if}

    <div class="buttons">
      <input type="text" name="chatId" bind:value={chatId}>
      <button class="btn" on:click={attemptConnect} disabled={ connecting ? true : null }>New Chat</button>
    </div>

  </div>
</div>

<style lang="scss">
  .home {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;

    background-color: var(--mg);
    color: #00000033;
    position: relative;
    overflow: hidden;

    .card {
      height: fit-content;
      display: flex;
      flex-direction: column;
      align-items: center;

      .error {
        font-size: 14px;
        font-family: Inter;
        font-weight: 500;
        color: #b06363;
      }

      .buttons {
        width: 100%;
        height: 36px;

        margin-top: 24px;

        display: flex;
        justify-content: space-around;

        .btn {
          height: 36px;
          padding: 0 12px;

          &[disabled] {
            opacity: .5;
            pointer-events: none;
          }
        }

        input {
          background-color: var(--mg);
          border: 2px solid var(--tb);
          border-radius: 6px;
          color: #ccc;
          padding: 0 12px 0 12px;
          margin-right: 12px;
          font-family: Inter;
          font-size: 0.85em;
          outline: none;

          &:hover {
            background-color: var(--fg);
            border: 2px solid var(--tb);
          }
        }
      }

      :global(svg) {
        max-width: 200px;
        max-height: 200px;
        color: #fff1;
      }
    }
  }
</style>
