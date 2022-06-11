<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "./Icon.svelte";

  let maximized: boolean = false;

  appWindow.listen('tauri://resize', async () => {
    maximized = await appWindow.isMaximized();
  });

  function minimize() { appWindow.minimize(); }
  function maximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }

</script>

<div class="titlebar" style="--maximized: { maximized ? '0px' : '1px' }" data-tauri-drag-region>
  <!-- Makes sure you can always drag the window -->
  {#if !maximized} <div class="drag-square" /> {/if}

  <div class="tab">
    <Icon name="chat" size="16px" />
    <span> Mjex </span>
  </div>
  <div class="tab">
    <Icon name="chatMsg" size="16px" />
    <span> Devensiv </span>
  </div>
  <div class="tab">
    <Icon name="chat" size="16px" />
    <span> Roboolet </span>
  </div>
  <div class="tab">
    <Icon name="chatMsg" size="16px" />
    <span> Funky </span>
  </div>

  <div class="button float-right" on:click={minimize}>
    <Icon name="minimize" />
  </div>
  <div class="button" on:click={maximize}>
    <Icon name="{ maximized ? 'unmaximize' : 'maximize' }" />
  </div>
  <div class="button close" on:click={close}>
    <Icon name="close" />
  </div>
</div>

<style lang="scss">

.titlebar {
  width: 100%;
  height: 44px;
  
  display: flex;
  align-items: center;
  z-index: 1;

  background-color: #1c1c1c;
  border-bottom: 1.5px solid #ffffff18;

  .drag-square {
    width: 45.6px;
    height: 44px;

    pointer-events: none;
  }

  .tab {
    // Tabs have 1 px offset when the window isn't maximized.
    margin-top: var(--maximized);

    min-width: 16px;
    min-height: 16px;
    height: 16px;

    display: flex;
    align-items: center;

    padding: 8px;
    margin-left: 4px;

    border-radius: 4px;
    border: 1.5px solid #ffffff22;
    color: #ffffff55;

    span {
      color: #bbb;
      margin-left: 8px;
      font-size: 0.7rem;
      font-family: Inter;
      margin-bottom: 1px;
      user-select: none;
    }
  }

  .button {
    width: 45.6px;
    height: 44px;

    display: flex;
    justify-content: center;
    align-items: center;

    outline: none;
    border: none;

    color: #fff;

    &:hover {
      background-color: #ffffff18;
    }

    &.close:hover {
      background-color: #c42b1c;
    }
  }
}

.float-right {
  margin-left: auto;
}

</style>