<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import Feed from "@lib/chat/Feed.svelte";
  import { onMount } from "svelte";
  import {
    getChatHistory,
    getUsername,
    insertChatHistory,
    onUsernameChanged,
  } from "@store";
  import { toPacket } from "@core/messages/Packet";
  import type { Msg } from "@core/messages/Msg";
  import { fade } from "svelte/transition";
  import { attachLemon, LemonEditor } from "lemon-editor";

  export let id: string;

  let editor: LemonEditor;
  let inputElement: HTMLDivElement;

  let myName = "Me";
  let peerName = "Peer";

  $: messages = [] as Msg[];
  $: id, updateHistory();

  onMount(async () => {
    // Retrieve the history for this chat.
    const history = await getChatHistory(id);
    if (history) messages = history;
    else messages = [];

    // Retrieve the username from the store.
    myName = getUsername() ?? "NoName";
    onUsernameChanged((newName) => {
      myName = newName;
      sendUsername();
    });
    sendUsername();

    editor = attachLemon({
      container: inputElement,
      content: "",
      onSubmit: sendMessage,
    });
  });

  async function updateHistory() {
    // Retrieve the history for this chat.
    const history = await getChatHistory(id);
    if (history) messages = history;
    else messages = [];
  }

  /* Listen for incoming messages from the peer */
  listen(`message_recieved_${id}`, (event) => {
    // Push the message into the messages array.
    let packet = toPacket(event as any);

    console.log("received packet: ", packet);

    if (packet.type == "Chat") {
      let time = getTime();

      // Push the message into the messages array.
      //messages.push({ sender: peerName, content: packet.content, time });

      // Update the persistent store.
      // insertChatHistory(id, {
      //   sender: peerName,
      //   content: packet.content,
      //   time,
      // });

      // Force update the Feed.
      messages = [...messages];
    }

    if (packet.type == "Username") {
      peerName = packet.content;
    }
  });

  /** Send a message to the peer */
  function sendMessage() {
    const time = getTime();
    const msg = editor.value;

    if (msg.trim().length === 0) return;

    // Push the message into the messages array.
    //messages.push({ sender: myName, content: msg, time });

    // Update the persistent store.
    //insertChatHistory(id, { sender: myName, content: msg, time });

    // Tell the backend to send the message.
    emit(`send_message_${id}`, { Chat: msg });

    // Force update the Feed.
    messages = [...messages];

    // Empty the input box.
    editor.clear();
  }

  /** Send a new username to the peer */
  function sendUsername() {
    emit(`send_message_${id}`, { Username: myName });
  }

  function getTime(): String {
    const now = new Date();
    const current =
      (now.getHours() < 10 ? "0" + now.getHours() : now.getHours()) +
      ":" +
      (now.getMinutes() < 10 ? "0" + now.getMinutes() : now.getMinutes());
    return current;
  }
</script>

<div class="chat" transition:fade={{ duration: 200 }}>
  <div class="toolbar">
    <div class="profile-picture" style="" />

    <span class="username">{peerName}</span>
  </div>

  <div class="logs">
    <Feed chat={messages} />
  </div>

  <div class="input">
    <div class="inputbox" bind:this={inputElement} />
  </div>
</div>

<style lang="scss">
  :global(.cm-scroller) {
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif !important;
  }

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
        font-family: LocalInter;
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
    }
  }
</style>
