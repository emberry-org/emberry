<script lang="ts">
  import Icon from "@lib/Icon.svelte";
  import { insertTab, navigateTo } from "@store";
  import { invoke } from "@tauri-apps/api/tauri";

  let chatId: string;
  let error: string = '';
  let connecting: boolean = false;

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

<div class="home">
  <div class="toolbar" />

  <div class="card">

    <Icon name="app/logo" size="40vh" />
    
    {#if error !== ''}
      <div class="error">{ error }</div>
    {/if}

    <div class="buttons">
      <input type="text" name="chatId" bind:value={chatId}>
      <button class="card-button start" on:click={attemptConnect} disabled={ connecting ? true : null }>New Chat</button>
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

    color: #00000033;
    position: relative;

    .toolbar {
      position: absolute;
      pointer-events: none;

      top: 0;
      left: 0;
      z-index: 2;

      width: 100%;
      height: 30.5px;

      background-color: #37383c;
      border-top: 1.5px solid #434547;
      border-bottom: 1.5px solid #434547;
    }

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

        .card-button {
          width: 30%;
          height: 36px;

          background-color: #37383c;
          border: 1px solid #545454;
          border-radius: 6px;

          padding: 0 12px 0 12px;

          font-family: Inter;
          font-size: 0.85em;
          color: #ccc;

          user-select: none;
          cursor: pointer;

          &.start {
            width: 40%;
            min-width: fit-content;
          }

          &:hover {
            border: 1px solid #5a5a5a;
          }

          &[disabled] {
            opacity: .5;
            pointer-events: none;
          }
        }

        input {
          background-color: #37383c;
          border: 1px solid #545454;
          border-radius: 6px;
          color: #ccc;
          padding: 0 12px 0 12px;
          margin-right: 12px;
          font-family: Inter;
          font-size: 0.85em;
          outline: none;

          &:hover {
            border: 1px solid #5a5a5a;
          }
        }
      }

      :global(svg) {
        max-width: 200px;
        max-height: 200px;
      }
    }
  }
</style>
