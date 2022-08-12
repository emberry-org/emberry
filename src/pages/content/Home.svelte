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

    color: #00000033;
    position: relative;
    overflow: hidden;

    &::before {
      content: "";
      position: absolute;
      width: 350px;
      height: 400px;
      pointer-events: none;
      border-radius: 50%;

      filter: blur(50px) saturate(150%);
      //background: radial-gradient(at 27% 37%,#3a8bfd 0,transparent 50%),radial-gradient(at 97% 21%,#9772fe 0,transparent 50%),radial-gradient(at 52% 99%,#fd3a4e 0,transparent 50%),radial-gradient(at 10% 29%,#5afc7d 0,transparent 50%),radial-gradient(at 97% 96%,#e4c795 0,transparent 50%),radial-gradient(at 33% 50%,#8ca8e8 0,transparent 50%),radial-gradient(at 79% 53%,#eea5ba 0,transparent 50%);
      background: radial-gradient(at 27% 37%,#fd833a 0,transparent 50%),radial-gradient(at 97% 21%,#feef72 0,transparent 50%),radial-gradient(at 52% 99%,#fd3a4e 0,transparent 50%),radial-gradient(at 10% 29%,#5afc7d 0,transparent 50%),radial-gradient(at 97% 96%,#e4c795 0,transparent 50%),radial-gradient(at 33% 50%,#e88c8c 0,transparent 50%),radial-gradient(at 79% 53%,#eea5ba 0,transparent 50%);
      background-repeat: no-repeat;
      background-position: center;
      opacity: .2;
    }

    // .toolbar {
    //   position: absolute;
    //   pointer-events: none;

    //   top: 0;
    //   left: 0;
    //   //z-index: 2;

    //   width: 100%;
    //   height: 30.5px;

    //   background-color: #37383a;
    //   border-top: 1.5px solid #fff2;
    //   border-bottom: 1.5px solid #fff1;
    // }

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
          background-color: var(--fg);
          border: 2px solid var(--tb);
          border-radius: 6px;
          color: #ccc;
          padding: 0 12px 0 12px;
          margin-right: 12px;
          font-family: Inter;
          font-size: 0.85em;
          outline: none;

          &:hover {
            background-color: var(--ffg);
            border: 2px solid var(--tb);
          }
        }
      }

      :global(svg) {
        max-width: 200px;
        max-height: 200px;
        color: var(--ffg);
      }
    }
  }
</style>
