<!-- User.Me : a compact panel for user account information (displayed in the top right corner) -->

<script lang="ts">
  import { onMount } from "svelte";
  import { getItem, onItem, setItem } from "../store";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event"

  let usernameInput: HTMLInputElement;
  let username: string = " ";

  onMount(() => {
    invoke("get_local").then((user: any) => {
      if (user === null) {
        username = "[no_user_pem]"
        return;
      }
      username = user.info.username;
      let local_id = user.identifier.bs58;
      listen(`usr_name_${local_id}`, (e: any) => {
        const name: string = e.payload;
        username = name
      });
    });
  });

  function keydown(evt: KeyboardEvent, input: HTMLInputElement) {
    if (evt.key === "Enter" && evt.shiftKey === false) {
      input.blur();
    }
  }

</script>


<div class="row">
  <div class="info">
    <input class="username" placeholder="Username" bind:this={usernameInput} bind:value={username} 
      on:blur={() => invoke("update_username", { name: username })} 
      on:keydown={(evt) => keydown(evt, usernameInput)} 
    />
  </div>
  <div class="avatar" />
</div>

<div class="row">
  <p class="desc">This is a place where you can add a description of your own :D</p>
</div>


<style lang="scss">

.row {
  height: 42px;
  display: flex;
  justify-content: flex-start;

  .avatar {
    width: 42px;
    min-width: 42px;
    height: 42px;

    background-color: #888;
    border-radius: 8px;
    margin-left: auto;
  }

  .info {
    display: flex;
    flex-direction: column;

    input {
      margin: 0;
      padding: 2px 4px;
      margin-left: -4px;
      background-color: #0000;
      outline: none;
      border-radius: 6px;
      border: none;
      font-family: inherit;

      &:focus, &:hover {
        background-color: #0004;
      }
    }

    .username {
      width: 90%;

      font-size: 16px;
      font-weight: 600;
      line-height: 17px;
      color: #eee;
    }
  }

  .desc {
    font-size: 14px;
    line-height: 21px;
    color: #aaa;
  }
}

</style>

