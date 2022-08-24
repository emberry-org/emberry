<script lang="ts">
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "@icon";
  import { oppSys } from "@store";

  $: maximized = false;
  $: hideDecorations = $oppSys == 'linux' || $oppSys == 'darwin';

  appWindow.listen('tauri://resize', async () => {
    maximized = await appWindow.isMaximized();
  });

  function minimize() { appWindow.minimize(); }
  function maximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }

</script>

<div class="titlebar" style="--maximized: { maximized || hideDecorations ? '0px' : '1px' }" hidden={hideDecorations} data-tauri-drag-region={!hideDecorations || null}>
  
  <div class="left" data-tauri-drag-region={!hideDecorations || null}>

  </div>

  <div class="right" data-tauri-drag-region={!hideDecorations || null}>

    <div class="button float-right" on:click={minimize}>
      <Icon name="window/minimize" />
    </div>
    <div class="button" on:click={maximize}>
      <Icon name="{ maximized ? 'window/unmaximize' : 'window/maximize' }" />
    </div>
    <div class="button close" on:click={close}>
      <Icon name="window/close" />
    </div>

  </div>

</div>

<style lang="scss">

.titlebar {
  width: 100vw;
  height: 30px;
  min-height: 30px;
  
  display: flex;
  align-items: flex-start;
  z-index: 2;

  background-color: transparent;
  border-bottom: 1.5px solid #333;

  .right {
    flex-grow: 1;
    height: 100%;

    display: flex;
  }

  // Titlebar Button
  .button {
    width: 46.5px;
    min-width: 46.5px;
    height: 30px;

    display: flex;
    justify-content: center;
    align-items: center;

    outline: none;
    border: none;
    z-index: 1;

    background-color: #fff0;
    color: #fff;

    &:hover {
      background-color: #fff1;
    }

    &.close:hover {
      background-color: #da2525;
    }
  }

  &[hidden] {
    .button {
      display: none;
    }
  }
}

</style>