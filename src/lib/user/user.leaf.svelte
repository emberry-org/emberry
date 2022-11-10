<!-- User.Leaf : a small wide display of a user (usually displayed within a list) -->

<script lang="ts">
  import { setItem } from "$lib/store";
  import { invoke } from "@tauri-apps/api/tauri";
  import { UserStatus, type User } from ".";

  /** The user this leaf belongs too */
  export let user: User;

  function onActivate() {
    switch (user.status) {

      /** If a pending user was clicked accept their request */
      case UserStatus.Pending:
        invoke('accept_room', {
          bs58cert: user.key,
          accepted: true,
        });
        // TODO: Need to check if actually connected !
        user.status = UserStatus.Connected;
        break;

      /** If a disconnected user was clicked try to connect */
      case UserStatus.Disconnected:
        tryRequest();
        break;

      /** If an offline user was clicked see if they're online */
      case UserStatus.Offline:
        tryRequest();
        break;
    
      default:
        break;

    }
  }

  /** Attempt to request a room with this user. */
  function tryRequest() {
    invoke("request_room", { bs58cert: user.key });
    setItem(user.key, JSON.stringify(UserStatus.Awaiting));
  }
</script>


<li class="leaf { user.status == UserStatus.Pending || user.status == UserStatus.Disconnected || user.status == UserStatus.Offline ? 'pending' : '' }" name={ user.name ?? 'unknown' } on:click={onActivate}>
  <p class="name">{ user.name ?? user.key }</p>
  <p class="status">{ UserStatus[user.status] }</p>
</li>


<style lang="scss">

.leaf {
  height: 42px;

  display: flex;
  align-items: center;

  padding: 0 12px;
  margin: 0 8px;

  border-radius: 8px;

  &:hover {
    background-color: #fff1;

    .status {
      display: none;
    }
  }

  &.pending {
    cursor: pointer;

    user-select: none;
    -webkit-user-select: none;
  }

  .name {
    text-overflow: ellipsis;
    overflow: hidden;

    padding-right: 12px;
    color: #ccc;
  }

  .status {
    font-size: 14px;
    color: #888;
  }
}

</style>

