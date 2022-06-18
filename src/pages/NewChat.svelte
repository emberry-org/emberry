<script lang="ts">
import { applicationTabs } from '@store';

  import { invoke } from '@tauri-apps/api/tauri'
  import { navigate } from "svelte-navigator";

  let id = '';
  let error = '';

  function keyPressed(e: KeyboardEvent) {
    if (e.key == 'Enter') {
      attemptConnect();
    }
  }

  function attemptConnect() {
    invoke('hole_punch', { peerKey: id }).then((id: string) => {
      // Create a new tab once the chat has been created.
      applicationTabs.update((tabs) => { tabs.push({ icon: 'chat', title: id.substring(0, 6), path: '/chat/' + id, keep_open: true }); return tabs; });
      // Navigate to the chat tab.
      navigate('/chat/' + id, { replace: true });
    }).catch((err) => {
      error = err;
      console.error('Failed to holepunch!', id, err);
    });
  }
</script>

<div class="chat">

  <div class="title">
    New Room
  </div>
  
  {#if error !== ''}
    <div class="error">{ error }</div>
  {/if}

  <div class="input">
    <input type="text" placeholder="Enter a user key" bind:value={id} on:keypress={keyPressed}>
  </div>

</div>

<style lang="scss">

.chat {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;
  justify-content: space-evenly;
  align-items: center;

  .title {
    font-size: 2.5em;
    font-family: Inter;
    font-weight: 700;
    color: #ffffff33;
  }

  .error {
    font-size: 14px;
    font-family: Inter;
    font-weight: 500;
    color: #b06363;
  }

  .input {
    width: 100%;
    height: 64px;

    display: flex;
    justify-content: center;
    align-items: flex-start;
    
    input {
      width: 50vw;
      max-width: 512px;
      height: 32px; 

      margin-top: 8px;
      padding: 0 12px 0 12px;
      box-shadow: 0 1px 2px 0 #00000055;

      background-color: #37383c;
      border: 1.5px solid #454749;
      outline: none;
      border-radius: 4px;

      color: #ccc;
      font-size: 1rem;
    }
  }
}

</style>