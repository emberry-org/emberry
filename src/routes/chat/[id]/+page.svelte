<script lang="ts">
  import { onItem } from "$lib/store";
  import { storeUsername } from "$lib/user";
  import { emit, listen } from "@tauri-apps/api/event"
  import { onMount } from "svelte";
  import { tick } from "svelte";
  import Msg from "$lib/chat/msg.svelte";
  import type { Message } from "$lib/chat/msg";
  import { invoke } from "@tauri-apps/api/tauri";

  /** Chat ID format : 'peer_id:room_id' */
  /** @type {import('./$types').PageData} */
  export let data: any;

  let msg = "";

  let localname = "";
  let peername = "";

  const peer_id = (data.id as string).split(':')[0];
  const room_id = (data.id as string).split(':')[1];
  
  let messages: Message[] = [];

  let feed: HTMLOListElement;
  let input: HTMLInputElement;

  onMount(() => {
    invoke("get_usr_info", { bs58cert: peer_id}).then((info: any) => {
      peername = info.username;
      listen(`usr_name_${peer_id}`, (e: any) => {
        peername = e.payload; 
      });
    }, (err: any) => {
      console.error("Could not get peer username from backend: ", err);
    });

    // Load our local username from the storage.
    localname = onItem(localStorage, (val) => {
      localname = val ?? "DefaultUsername";
      // Send our new username to the peer.
      emit(`send_message_${room_id}`, { Username: localname });
    }, "username") ?? "DefaultUsername";

    // Listen for incoming messages.
    listen(`message_recieved_${room_id}`, (e: any) => {
      const type: string = Object.keys(e.payload.message)[0];

      const msg = { type, content: e.payload.message[type], sender: peername };
      addMessage(msg.content, peername);
    });

    // Send our username to the peer.
    emit(`send_message_${room_id}`, { Username: localname });

    // Set the list to scroll to the bottom of the messages.
    feed.scrollTop = feed.scrollHeight;

    // Send a message when the enter key is pressed.
    input.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === "Enter" && e.shiftKey === false) {
        sendMessage();
      }
    });
  });

  async function addMessage(content: any, sender: string) {
    // Get the time and chain.
    const date = new Date();
    const time = `Today at ${ date.getHours().toString().padStart(2, '0') }:${ date.getMinutes().toString().padStart(2, '0') }`;
    const chain = messages.length > 0 && messages[messages.length - 1].sender === sender;

    // Add the message to the feed.
    messages.push({ type: "Chat", content, sender, time, chain });
    messages = [...messages];

    // Check if the user has scrolled all the way to the bottom.
    if (feed.scrollTop !== feed.scrollHeight - feed.clientHeight) return;
    await tick(); // Wait a tick for the UI to update.

    // Move the list up with the new message that was added.
    feed.scrollTop = feed.scrollHeight;
  }

  async function sendMessage() {
    // Don't send anything if the message is whitespace.
    if (msg.trim().length === 0) return;

    // Send the message and add it to our own feed.
    emit(`send_message_${room_id}`, { Chat: msg });
    addMessage(msg, localname);

    // Empty the input box.
    msg = "";
  }
</script>


<div class="header">
  <h2>{ peername }</h2>
</div>

<div class="chat">

  <div class="feed">
    <ol bind:this={ feed }>
      {#each messages as message}
    
      <Msg 
        sender={message.sender} 
        content={message.content} 
        time={message.time} 
        chain={message.chain} 
      />

      <!-- <li>
        { message.sender } : { message.content }
      </li> -->

      {/each}
    </ol>
  </div>

  <div class="bar">

    <input class="default" placeholder="Enter a message..." bind:value={msg} bind:this={input} />
    <button class="default" on:click={sendMessage}>
      Send Msg
    </button>

  </div>

</div>


<style lang="scss">

.chat {
  display: flex;
  flex-direction: column;
  padding: 12px;

  .feed {
    position: relative;
    flex-grow: 1;
    overflow: none;

    ol {
      max-height: calc(100% - 12px);
      width: 100%;
      overflow: auto;
      position: absolute;
      bottom: 0;

      list-style: none;
      padding: 12px;
      margin: 0;

      &::-webkit-scrollbar {
        display: none;
      }
    }
  }

  .bar {
    height: 42px;
    display: flex;
  }
}

h2 {
  overflow-wrap: break-word;
  font-size: 20px;

  padding: 0 16px;
  color: #eee;
}

input {
  width: 100%;
  margin-right: 12px;

  background-color: #1F1F1F;
  color: #eee;
  
  &::placeholder {
    color: #fff5;
  }
}

button {
  width: 160px;
  white-space: nowrap;

  background-color: #1F1F1F;
  color: #eee;
}

</style>

