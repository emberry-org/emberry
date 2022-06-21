<script lang="ts">
  import Icon from "@lib/Icon.svelte";
  import { insertTab, navigateTo } from "@store";
  import { invoke } from "@tauri-apps/api/tauri";

  let chatId: string;
  let error: string = '';
  let connecting: boolean = false;

  function attemptConnect() {
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
  <div class="card">

    <Icon name="logo" size="50vh" />
    
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

    color: #00000028;

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

          background-color: #3d3a3a;
          border: 1.5px solid #ffffff18;
          box-shadow: 0 1px 2px 0 #00000055;
          border-radius: 6px;

          padding: 0 12px 0 12px;

          font-family: Inter;
          font-size: 0.85em;
          color: #efdab9;

          user-select: none;
          cursor: pointer;

          &.start {
            width: 40%;
            min-width: fit-content;
          }

          &:hover {
            border: 1.5px solid #ffffff28;
          }

          &[disabled] {
            opacity: .5;
            pointer-events: none;
          }
        }

        input {
          background-color: #3d3a3a;
          border: 1.5px solid #ffffff18;
          box-shadow: 0 1px 2px 0 #00000055;
          border-radius: 6px;
          color: #efdab9;
          padding: 0 12px 0 12px;
          margin-right: 12px;
          font-family: Inter;
          font-size: 0.85em;
          outline: none;

          &:hover {
            border: 1.5px solid #ffffff28;
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
