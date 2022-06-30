<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "@lib/Icon.svelte";
  import { getSelectedTab, getTabs, navigateTo, onTabsChange, onTabSelected, oppSys } from '@store';
  import { onMount } from 'svelte';
  import type AppTab from '@core/AppTab';
  import Pub from './Pub.svelte';

  $: maximized = false;
  $: hideDecorations = $oppSys == 'linux' || $oppSys == 'darwin';

  $: tabs = [] as AppTab[];
  $: selected = '/';

  let ready = false;

  onMount(async () => {
    // Setup the tabs:
    const storedTabs = getTabs();
    if (storedTabs) tabs = storedTabs; else tabs = [];

    onTabsChange((storedTabs) => {
      tabs = storedTabs;
    });

    // Setup the selected tab:
    const selectedTab = await getSelectedTab();
    if (selectedTab) selected = selectedTab; else selected = '/';

    onTabSelected((path: string) => {
      selected = path;
    });

    ready = true;
  });

  appWindow.listen('tauri://resize', async () => {
    maximized = await appWindow.isMaximized();
  });

  function minimize() { appWindow.minimize(); }
  function maximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }

</script>

<div class="titlebar" style="--maximized: { maximized || hideDecorations ? '0px' : '1px' }" hidden={hideDecorations}>
  
  <div class="left" data-tauri-drag-region={!hideDecorations || null}>
    <!-- Makes sure you can always drag the window -->
    {#if !maximized && !hideDecorations} <div class="drag-square" /> {/if}

    <div class="profile">
      <div class="img" style="background-image: url(https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=48)" />
      <div class="info">
        <h3>Username</h3>
        <p>#1234</p>
      </div>
    </div>
  </div>

  <div class="right" data-tauri-drag-region={!hideDecorations || null}>

    <Pub />

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
  height: 52px;
  min-height: 52px;
  
  display: flex;
  align-items: flex-start;
  z-index: 1;

  .left {
    width: 248px;
    min-width: 248px;
    height: 100%;

    display: flex;

    .drag-square {
      width: 45.6px;
      min-width: 45.6px;
      height: 44px;

      pointer-events: none;
    }

    .profile {
      width: 128px;
      height: 100%;

      margin-left: 8px;

      display: flex;
      align-items: center;

      .img {
        width: 34px;
        height: 34px;

        margin-right: 8px;
        position: relative;

        background-size: contain;
        border-radius: 8px;

        &::after {
          content: "";
          
          width: 30px;
          height: 30px;

          position: absolute;
          top: 0;
          left: 0;

          border: 2px solid #ffffff10;
          border-radius: 8px;
        }
      }

      .info {
        flex-grow: 1;
        height: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;

        h3 {
          margin: 0;
          font-size: 0.9em;
        }

        p {
          margin: 0;
          font-size: 0.7em;
        }
      }
    }
  }

  .right {
    flex-grow: 1;
    height: 100%;

    display: flex;
    background-color: #1e2024;
    border-left: 1.5px solid #ffffff16;
  }

  .button {
    width: 46.5px;
    min-width: 46.5px;
    height: 28px;

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
      background-color: #da2525;
    }
  }

  &[hidden] {
    background-color: #343434;

    .button {
      display: none;
    }
  }
}

</style>