<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { setItem } from "./store";
  import { UserStatus } from "./user";

  let usrkey = "";

  function send() {
    invoke("request_room", { bs58cert: usrkey });
    setItem(usrkey, JSON.stringify(UserStatus.Awaiting));
  }
</script>

<section class="body">
  <div class="col">
    <input class="default" placeholder="Enter a user key..." bind:value={usrkey} />
    <button class="default" disabled={usrkey.length <= 0} on:click={send}>
      Send Request
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
  background-color: #111;
  color: #fff;
  
  width: 100%;
}

input::placeholder {
  color: #686868;
}

button {
  background-color: #0000;
  color: #EA8A44;
  font-weight: 700 !important;

  width: fit-content;
  margin-top: 16px;

  border: 1.5px solid #EA8A44 !important;
}

button:hover {
  background-color: #EA8A44;
  color: #111;

  border: 1.5px solid #000 !important;
}

button:disabled {
  background-color: #0000;
  border: 1.5px solid #383838 !important;
  color: #484848;
  cursor: default;
}

</style>

