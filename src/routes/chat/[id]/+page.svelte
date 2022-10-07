<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event"
  import { onMount } from "svelte";
  import { tick } from "svelte";

  /** @type {import('./$types').PageData} */
  export let data: any;

  let msg = "";
  let messages: any[] = [];

  let feed: HTMLOListElement;
  let input: HTMLInputElement;

  onMount(() => {
    // Listen for incoming messages.
    listen(`message_recieved_${data.id}`, (e: any) => {
      const type: string = Object.keys(e.payload.message)[0];
      const msg = { type, content: e.payload.message[type], sender: 'Them' };

      addMessage(msg.content, "Them");

      console.log("received msg: ", msg);
    });

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
    // Add the message to the feed.
    messages.push({ type: "Chat", content, sender });
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
    emit(`send_message_${data.id}`, { Chat: msg });
    addMessage(msg, "Me");

    // Empty the input box.
    msg = "";
  }
</script>


<div class="header">
  <h2>{ data.id }</h2>
</div>

<div class="chat">

  <div class="feed">
    <ol bind:this={ feed }>
      {#each messages as message}
    
      <li>
        { message.sender } : { message.content }
      </li>

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

      li {
        color: #eee;
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

  background-color: #1F1F1F;
  color: #eee;
}

</style>

