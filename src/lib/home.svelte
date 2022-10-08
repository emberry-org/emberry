<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { UserStatus } from "./user.status";
  import { storeUser } from "./user.store";

  let usrkey = "";

  function send() {
    let utf8Encode = new TextEncoder();
    invoke("request_room", { usr: { key: Array.from(utf8Encode.encode(usrkey)) } });
    storeUser({ key: usrkey, status: UserStatus.Pending });
  }
</script>

<section class="body">
  <div class="col">
    <input class="default" placeholder="Enter a user key..." bind:value={usrkey} />
    <button on:click={send}>
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
  width: 100%;
}

button {
  width: fit-content;
  margin-top: 16px;
}

</style>

