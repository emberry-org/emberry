<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "@lib/Icon.svelte";

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

  <div class="tabs">
    <div class="tab">
      <Icon name="chat" size="16px" />
      <span> Mjex </span>
    </div>
    <div class="tab">
      <Icon name="chatMsg" size="16px" />
      <span> Devensiv </span>
    </div>
    <div class="tab" selected>
      <Icon name="chat" size="16px" />
      <span> Roboolet </span>
    </div>
    <div class="tab">
      <Icon name="chatMsg" size="16px" />
      <span> Funky </span>
    </div>
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
  width: 100vw;
  height: 44px;
  
  display: flex;
  align-items: center;
  z-index: 1;

  background-color: #212327;
  border-bottom: 1.5px solid #434547;

  .drag-square {
    width: 45.6px;
    min-width: 45.6px;
    height: 44px;

    pointer-events: none;
  }

  .tabs {
    flex-grow: 1;
    height: 44px;

    display: flex;
    align-items: center;
    overflow: hidden;
    pointer-events: none;

    .tab {
      // Tabs have 1 px offset when the window isn't maximized.
      margin-top: var(--maximized);

      min-width: fit-content;
      min-height: 16px;
      height: 16px;

      display: flex;
      align-items: center;

      padding: 8px;
      margin-left: 4px;

      pointer-events: all;
      border-radius: 4px;
      cursor: pointer;
      color: #737578;

      :global(svg) {
        min-width: 16px;
        min-height: 16px;
      }

      span {
        color: #bbb;
        margin-left: 8px;
        font-size: 0.7rem;
        font-family: Inter;
        margin-bottom: 1px;
        user-select: none;
      }

      &:hover {
        background-color: #ffffff11;
      }

      &[selected] {
        background-color: #00000044;
        color: #636568;
        cursor: default;
      }
    }
  }

  .button {
    width: 45.6px;
    min-width: 45.6px;
    height: 44px;

    display: flex;
    justify-content: center;
    align-items: center;

    outline: none;
    border: none;

    color: #fff;

    &:hover {
      background-color: #ffffff11;
    }

    &.close:hover {
      background-color: #c42b1c;
    }
  }
}

</style>