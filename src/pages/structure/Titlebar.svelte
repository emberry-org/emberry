<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "@lib/Icon.svelte";
  import { getSelectedTab, getTabs, navigateTo, onTabsChange, onTabSelected, oppSys } from '@store';
  import { onMount } from 'svelte';
  import type AppTab from '@core/AppTab';

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

<div class="titlebar" style="--maximized: { maximized || hideDecorations ? '0px' : '1px' }" hidden={hideDecorations} data-tauri-drag-region={!hideDecorations || null}>
  <!-- Makes sure you can always drag the window -->
  {#if !maximized && !hideDecorations} <div class="drag-square" /> {/if}

  <!-- <div class="tabs { ready ? '' : 'hidden' }">
    <div class="tab home" on:click={() => navigateTo('/')} selected={selected == '/' ? true : null}>
      <Icon name="home" size="16px" />
    </div>
    {#each tabs as tab}
      <div class="tab" on:click={() => navigateTo(tab.path)} selected={selected == tab.path ? true : null}>
        <Icon name={tab.icon} size="16px" />
        <span style="font-style: { tab.keep_open ? '' : 'italic' };"> { tab.title } </span>
      </div>
    {/each}
  </div> -->

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

  background-color: #222222;

  .drag-square {
    width: 45.6px;
    min-width: 45.6px;
    height: 44px;

    pointer-events: none;
  }

  // .tabs {
  //   flex-grow: 1;
  //   height: 44px;

  //   display: flex;
  //   align-items: center;
  //   overflow: hidden;
  //   pointer-events: none;

  //   &.hidden {
  //     display: none;
  //   }

  //   .tab {
  //     // Tabs have 1 px offset when the window isn't maximized.
  //     margin-top: var(--maximized);

  //     min-width: fit-content;
  //     min-height: 16px;
  //     height: 16px;

  //     display: flex;
  //     align-items: center;

  //     padding: 8px;
  //     margin-left: 4px;

  //     pointer-events: all;
  //     border-radius: 4px;
  //     cursor: pointer;
  //     color: #efdab9;

  //     :global(svg) {
  //       min-width: 16px;
  //       min-height: 16px;
  //     }

  //     span {
  //       color: #bbb;
  //       margin-left: 8px;
  //       font-size: 0.7rem;
  //       font-family: Inter;
  //       user-select: none;
  //       -webkit-user-select: none;
  //     }

  //     &:hover {
  //       background-color: #ffffff11;
  //     }

  //     &.home {
  //       margin-left: 8px;

  //       :global(svg) {
  //         margin-bottom: 2px;
  //       }
  //     }

  //     &[selected] {
  //       background-color: #00000033;
  //       color: #efdab988;
  //       cursor: default;
  //     }
  //   }
  // }

  .button {
    width: 46.5px;
    min-width: 46.5px;
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
      background-color: #c13f21;
    }
  }

  &[hidden] {
    .button {
      display: none;
    }
  }
}

</style>