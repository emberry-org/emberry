<script lang="ts">
  import { onMount } from "svelte";
  import { tick } from "svelte";
  import Msg from "lib/chat/msg.svelte";
  import type { Message } from "lib/chat/msg";
  import { getLocalUserInfo, getUserInfo, onUserInfo } from "comms/warehouse";
  import { onMessage, sendMessage, sendUsername } from "comms/msg";

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

  onMount(async () => {
    /* Setup the online user */
    getUserInfo(peer_id).then((info) => { peername = info.name; });
    onUserInfo(peer_id, (e) => { if (e.name) peername = e.name; });

    /* Setup the local user */
    const user = await getLocalUserInfo();

    localname = user.name;
    sendUsername(room_id, localname);

    onUserInfo(user.id, (e) => {
      if (e.name) localname = e.name;
      sendUsername(room_id, localname);
    });

    /* Listen for incoming messages */
    onMessage(room_id, (e) => {
      if (e.msg.type === "Chat") addMessage(e.msg.content, peername);
    });

    // Set the list to scroll to the bottom of the messages.
    feed.scrollTop = feed.scrollHeight;

    // Send a message when the enter key is pressed.
    input.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === "Enter" && e.shiftKey === false) send();
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
    if (feed.scrollTop < feed.scrollHeight - feed.clientHeight - 100) return;
    await tick(); // Wait a tick for the UI to update.

    // Move the list up with the new message that was added.
    feed.scrollTop = feed.scrollHeight;
  }

  async function send() {
    if (msg.trim().length === 0) return;

    // Send the message and add it to our own feed.
    sendMessage(room_id, { Chat: msg });
    addMessage(msg, localname);

    // Empty the input box.
    msg = "";
  }
</script>


<div class="header">
  <h2>{ peername } <span class="id">@0000</span></h2>
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
    <button class="default" on:click={send}>
      Send Msg
    </button>

  </div>

</div>


<style lang="scss">

.chat {
  display: flex;
  flex-direction: column;
  padding: 12px 12px 12px 0;

  .feed {
    position: relative;
    flex-grow: 1;
    overflow: hidden;

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
  font-weight: 800;

  padding: 0 20px;
  color: var(--text-base);

  .id {
    color: var(--text-secondary);
    font-weight: 400;
  }
}

input {
  width: 100%;
  margin: 0 12px;

  background-color: var(--bg-100);
  color: var(--text-base);
  
  &::placeholder {
    color: var(--text-secondary-light);
  }
}

button {
  width: 160px;
  white-space: nowrap;

  background-color: var(--bg-100);
  color: var(--text-base);
}

</style>

