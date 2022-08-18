<script lang="ts">
  import { emit, listen } from '@tauri-apps/api/event'
  import Feed from "@lib/chat/Feed.svelte";
  import { onMount } from 'svelte';
  import { getChatHistory, getUsername, insertChatHistory, onUsernameChanged } from '@store';
  import { toPacket } from '@core/messages/Packet';
  import type Msg from '@core/messages/Msg';
  import { fade } from 'svelte/transition';
  import { Editor, Extension } from '@tiptap/core'
  import { Bold } from '@core/tiptap-ext/bold';
  import { Text } from '@core/tiptap-ext/text';
  import { Document } from '@core/tiptap-ext/doc';
  import { Paragraph } from '@core/tiptap-ext/p';

  export let id: string;

  let input: Editor;
  let inputElement: HTMLDivElement;
  
  let myName = 'Me';
  let peerName = 'Peer';

  $: messages = [] as Msg[];
  $: id, updateHistory();

  onMount(async () => {
    // Retrieve the history for this chat.
    const history = await getChatHistory(id);
    if (history) messages = history; else messages = [];

    // Retrieve the username from the store.
    myName = getUsername() ?? "NoName";
    onUsernameChanged((newName => { myName = newName; sendUsername(); }));
    sendUsername();

    input = new Editor({
      element: inputElement,
      extensions: [
        Text,
        Document,
        Paragraph,
        Bold,

        new class extends Extension {
          keys() {
            return {
              Enter(state, dispatch, view) {
                sendMessage();
                console.log('test');
                // return true prevents default behaviour
                return true
              },
            }
          }
        }() as any,
      ],
      content: `<p>Hello world!</p>`,
    });
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
    if (e.key == 'Enter' && e.shiftKey == false) { e.preventDefault(); sendMessage(); }
  }

  /** Send a message to the peer */
  function sendMessage() {
    const time = getTime();
    const msg = input.getText();

    // Push the message into the messages array.
    messages.push({ sender: myName, content: msg, time });

    // Update the persistent store.
    insertChatHistory(id, { sender: myName, content: msg, time });

    // Tell the backend to send the message.
    emit(`send_message_${id}`, { type: 'Chat', content: msg });

    // Force update the Feed.
    messages = [...messages];

    // Empty the input box.
    input.commands.clearContent();
  }

  /** Send a new username to the peer */
  function sendUsername() {
    emit(`send_message_${id}`, { type: 'Username', content: myName });
  }

  function getTime(): String {
    const now = new Date();
    const current = (now.getHours() < 10 ? '0' + now.getHours() : now.getHours()) + ':' + (now.getMinutes() < 10 ? '0' + now.getMinutes() : now.getMinutes());
    return current;
  }

</script>

<div class="chat" transition:fade={{ duration: 200 }}>

  <div class="toolbar">
    <div class="profile-picture" style="" />

    <span class="username">{ peerName }</span>
  </div>

  <div class="logs">
    <Feed chat={ messages } />
  </div>

  <div class="input">
    <!-- <input type="text" bind:value={ inputBox } on:keypress={ keyPressed }> -->
    <!-- <div class="inputbox" contenteditable="true" bind:this={inputElement} on:keypress={keyPressed} /> -->
    <div class="inputbox" bind:this={inputElement} />
  </div>

</div>

<style lang="scss">

.chat {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;
  position: absolute;
  background-color: var(--fg);

  .toolbar {
    pointer-events: all;

    width: 100%;
    height: 52px;
    min-height: 52px;
    margin-top: 7px;

    display: flex;
    align-items: center;

    .profile-picture {
      margin-left: 20px;
    }

    .username {
      font-family: Inter;
      font-weight: 500;
      font-size: 16px;
      color: #ddd;

      margin-left: 20px;
      overflow: hidden;
      pointer-events: all;
      background-color: transparent;
      outline: none;
      border: none;

      user-select: none;
      -webkit-user-select: none;
    }
  }

  .logs {
    width: 100%;

    display: flex;
    justify-content: center;
  }

  .input {
    width: 100%;
    height: 64px;

    display: flex;
    justify-content: center;
    align-items: flex-start;
    
    // input {
    //   width: calc(100% - 64px);
    //   height: 32px; 

    //   font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica,
    //   Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";

    //   margin-top: 8px;
    //   padding: 2px 12px 2px 12px;
    //   box-shadow: 0 1px 2px 0 #00000055;

    //   background-color: #37383a;
    //   border: 2px solid #ffffff11;
    //   outline: none;
    //   border-radius: 4px;

    //   color: #ccc;
    //   font-size: 1rem;
    // }
  }
}

</style>