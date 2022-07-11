<script lang="ts">
  import { emit, listen } from '@tauri-apps/api/event'
  import Feed from "@lib/chat/Feed.svelte";
  import { onMount } from 'svelte';
  import { getChatHistory, getUsername, insertChatHistory, onUsernameChanged } from '@store';
  import { toPacket } from '@core/messages/Packet';
  import type Msg from '@core/messages/Msg';

  export let id: string;

  let inputBox = '';
  
  let myName = 'Me';
  let peerName = 'Peer';

  $: messages = [] as Msg[];
  $: id, updateHistory();

  onMount(async () => {
    // Retrieve the history for this chat.
    const history = await getChatHistory(id);
    if (history) messages = history; else messages = [];

    // Retrieve the username from the store.
    myName = getUsername();
    onUsernameChanged((newName => { myName = newName; sendUsername(); }));
    sendUsername();
  });

  async function updateHistory() {
    // Retrieve the history for this chat.
    const history = await getChatHistory(id);
    if (history) messages = history; else messages = [];
  }

  /* Listen for incoming messages from the peer */
  listen(`message_recieved_${id}`, (event) => {
    // Push the message into the messages array.
    let packet = toPacket(event as any);

    console.log('received packet: ', packet);

    if (packet.type == 'Chat') {
      let time = getTime();
      
      // Push the message into the messages array.
      messages.push({ sender: peerName, content: packet.content, time });

      // Update the persistent store.
      insertChatHistory(id, { sender: peerName, content: packet.content, time });

      // Force update the Feed.
      messages = [...messages];
    }

    if (packet.type == 'Username') {
      peerName = packet.content;
    }
  });

  /** Listen for the user to press the enter key */
  function keyPressed(e: KeyboardEvent) {
    if (e.key == 'Enter') { sendMessage(); }
  }

  /** Send a message to the peer */
  function sendMessage() {
    let time = getTime();

    // Push the message into the messages array.
    messages.push({ sender: myName, content: inputBox, time });

    // Update the persistent store.
    insertChatHistory(id, { sender: myName, content: inputBox, time });

    // Tell the backend to send the message.
    emit(`send_message_${id}`, { type: 'Chat', content: inputBox });

    // Force update the Feed.
    messages = [...messages];

    // Empty the input box.
    inputBox = '';
  }

  /** Send a new username to the peer */
  function sendUsername() {
    emit(`send_message_${id}`, { type: 'Username', content: myName });
  }

  function getTime(): String {
    const now = new Date();
    const current = now.getHours() + ':' + now.getMinutes();
    return current;
  }

</script>

<div class="chat">

  <div class="toolbar">
    <span class="username">{ peerName }</span>
  </div>

  <div class="logs">
    <Feed chat={ messages } />
  </div>

  <div class="input">
    <input type="text" bind:value={ inputBox } on:keypress={ keyPressed }>
  </div>

</div>

<style lang="scss">

.chat {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;
  position: relative;

  .toolbar {
    position: absolute;
    pointer-events: all;

    top: 0;
    left: 0;
    //z-index: 2;

    width: 100%;
    height: 30.5px;

    background-color: #373937;
    border-top: 1.5px solid #fff2;
    border-bottom: 1.5px solid #fff1;

    display: flex;
    align-items: center;

    .username {
      font-family: Inter;
      font-weight: 500;
      font-size: 14px;
      color: #ddd;

      margin-left: 10px;
      overflow: hidden;
      pointer-events: all;
      background-color: transparent;
      outline: none;
      border: none;
    }
  }

  .logs {
    width: 100%;
    flex-grow: 1;

    display: flex;
    justify-content: center;
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
      border: 1.5px solid #ffffff11;
      outline: none;
      border-radius: 4px;

      color: #ccc;
      font-size: 1rem;
    }
  }
}

</style>