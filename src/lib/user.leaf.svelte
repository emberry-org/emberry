<!-- User.Leaf : a small wide display of a user (usually displayed within a list) -->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { User } from "./user";
  import { UserStatus } from "./user.status";

  /** The user this leaf belongs too */
  export let user: User;

  function onActivate() {
    if (user.status == UserStatus.Pending) {
      invoke('accept_room', {
        usr: { key: user.key },
        accepted: true,
      });
      // todo : Need to check if actually connected !
      user.status = UserStatus.Connected;
    }
  }
</script>


<li class="leaf { user.status == UserStatus.Pending ? 'pending' : '' }" name={ user.name ?? 'unknown' } on:click={onActivate}>
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

