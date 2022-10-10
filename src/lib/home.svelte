<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { UserStatus, storeUser } from "./user";

  let usrkey = "";

  function send() {
    let utf8Encode = new TextEncoder();
    invoke("request_room", { usr: { key: Array.from(utf8Encode.encode(usrkey)) } }).catch(() => {
      console.log('user not found!');
    }).then(() => {
      storeUser({ key: usrkey, status: UserStatus.Awaiting });
    });
  }
</script>

<section class="body">
  <div class="col">
    <input class="default" placeholder="Enter a user key..." bind:value={usrkey} />
    <button class="default" on:click={send}>
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

