<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "./Icon.svelte";

  let maximized: boolean = false;

  function minimize() { appWindow.minimize(); }
  async function maximize() { appWindow.toggleMaximize(); maximized = await appWindow.isMaximized(); }
  function close() { appWindow.close(); }

</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="button" on:click={minimize}>
    <Icon name="minimize" strokeWidth="0em" />
  </div>
  <div class="button" on:click={maximize}>
    <Icon name="maximize" strokeWidth="0em" />
  </div>
  <div class="button close" on:click={close}>
    <Icon name="close" strokeWidth="0em" />
  </div>
</div>

<style lang="scss">

.titlebar {
  width: 100%;
  height: 40px;
  
  display: flex;
  justify-content: flex-end;

  background-color: #1c1c1c;

  .button {
    width: 45.6px;
    height: 40px;

    display: flex;
    justify-content: center;
    align-items: center;

    outline: none;
    border: none;

    color: white;

    &:hover {
      background-color: #ffffff18;
    }

    &.close:hover {
      background-color: #c42b1c;
    }
  }
}

</style>