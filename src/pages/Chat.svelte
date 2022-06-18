<script lang="ts">
  import { emit, listen } from '@tauri-apps/api/event'
  import Feed from "@lib/chat/Feed.svelte";
  //import Icon from "@lib/Icon.svelte";
  //import { addressBookState } from "@store";
  //import { onMount } from "svelte";

  export let id: string;

  let message = '';

  $: messages = [] as { sender: String, content: String }[];

  listen(`message_recieved_${id}`, (event) => {
    let message = (event.payload as any).message as string;
    messages.push({ sender: 'Peer', content: message });
  });

  function keyPressed(e: KeyboardEvent) {
    if (e.key == 'Enter') {
      sendMessage();
    }
  }

  function sendMessage() {
    messages.push({ sender: 'Me', content: message });
    emit(`send_message_${id}`, message);
    message = '';
  }

  //function toggleAddressBook() { addressBookState.set(!$addressBookState); }

</script>

<div class="chat">

  <div class="toolbar">
    <!-- <button class="icon-button" on:click={toggleAddressBook}>
      <Icon name="addressBook" size="16px" />
    </button>
    <div class="seperator" /> -->
    <div class="username">Roboolet</div>
  </div>

  <div class="logs">
    <Feed chat={messages} />
  </div>

  <div class="input">
    <input type="text" bind:value={message} on:keypress={keyPressed}>
  </div>

</div>

<style lang="scss">

.chat {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;

  .toolbar {
    width: 100%;
    height: 32px;

    display: flex;
    align-items: center;

    background-color: #37383c;
    border-bottom: 1.5px solid #434547;
    box-shadow: 0 1px 2px 0 #00000055;

    button {
      margin-left: 4px;
      margin-right: 4px;
      padding-bottom: 1px;
    }

    .seperator {
      width: 0px;
      height: 65%;

      border-right: 1.5px solid #ffffff18;
    }

    .username {
      font-family: Inter;
      font-weight: 500;
      font-size: 0.9rem;
      color: #aaa;

      margin-left: 10px;
    }
  }

  .logs {
    width: 100%;
    flex-grow: 1;
  }

  .input {
    width: 100%;
    height: 64px;

    display: flex;
    justify-content: center;
    align-items: flex-start;
    
    input {
      width: calc(100% - 32px);
      max-width: 50vw;
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