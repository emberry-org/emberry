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
      // TODO: This should be checked
      user.status = UserStatus.Connected;
    }
  }

</script>


<li class="leaf" name={ user.name ?? 'unknown' } on:click={onActivate}>
  { user.name ?? 'unknown' }
</li>


<style lang="scss">

.leaf {
  height: 36px;

  display: flex;
  align-items: center;

  padding: 0 12px;
  cursor: pointer;

  &:hover {
    background-color: #fff1;
  }
}

</style>

