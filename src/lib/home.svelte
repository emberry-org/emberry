<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  function greet() {
    let utf8Encode = new TextEncoder();
    invoke("request_room", { usr: { key: Array.from(utf8Encode.encode(name)) } })
    console.log('send room request to : ' + name);
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

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

input,
button {
  outline: none;
}

</style>

