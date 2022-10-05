<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet() {
    let utf8Encode = new TextEncoder();
    invoke("request_room", { usr: { key: Array.from(utf8Encode.encode(name)) } })
  }
</script>

<section class="body">
  <div class="col">
    <input id="greet-input" placeholder="Enter a user key..." bind:value={name} />
    <button on:click={greet}>
      Send Request
    </button>
  </div>
  <p>{greetMsg}</p>
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