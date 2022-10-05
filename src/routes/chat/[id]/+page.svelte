<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit } from "@tauri-apps/api/event"

  /** @type {import('./$types').PageData} */
  export let data;

  let msg = "";

  async function send() {
    emit(`send_message_${id}`, { Chat: msg });
    msg = "";
  }
</script>

<section class="body">
  <div class="col">
    <h2>{ data.id }</h2>
    <input id="greet-input" placeholder="Enter a message..." bind:value={msg} />
    <button on:click={send}>
      Send Msg
    </button>
  </div>
</section>

<style>

.body {
  justify-content: center;
  align-items: center;
}

.col {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

input {
  width: 100%;
}

button {
  width: fit-content;
  margin-top: 16px;
}

</style>