<!-- User.Me : a compact panel for user account information (displayed in the top right corner) -->

<script lang="ts">
  import { onMount } from "svelte";
  import { getLocalUserInfo, onUserInfo, setUsername } from "comms/warehouse";

  let usernameInput: HTMLInputElement;
  let username: string = " ";
  let bs58cert: string = " ";

  onMount(async () => {
    const user = await getLocalUserInfo();
    username = user.name;

    /* Check if the local user is set */
    if (user.id === "unknown") { return; }

    bs58cert = user.id;

    onUserInfo(user.id, (e) => {
      username = e.name;
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
      on:blur={() => setUsername(username)} 
      on:keydown={(evt) => keydown(evt, usernameInput)} 
    />
  </div>
  <div class="avatar" />
</div>

<div class="row">
  <p class="desc">{ bs58cert }</p>
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

