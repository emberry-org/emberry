<script lang="ts">
  import { Body, Titlebar, Statusbar } from "@win";
  import { CommandCenter, Snackbar } from '@lib/overlays';
  import { commandCenterState } from "@store";
  import setupOS from "@core/utils/OppSys";
  import { onMount } from "svelte";

  onMount(() => {
    setupOS();
  });

  /** Check for local shortcuts */
  document.addEventListener("keydown", function(e) {
    if (e.ctrlKey && e.shiftKey && e.code === "KeyP") {
      e.preventDefault();
      commandCenterState.update(state => !state);
    }
  });
</script>

<main>

  <div class="floating">
    <Snackbar />

    {#if $commandCenterState}
      <CommandCenter />
    {/if}
  </div>
  
  <div class="contents">
    <Titlebar />

    <Body />

    <Statusbar />
  </div>

</main>

<style lang="scss" global>
  @import './style/vars.scss';
  @import './style/fonts.css';
  @import './style/global.scss';

  :root {
    font-family: Amulya, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  html,
  body {
    padding: 0;
    margin: 0;
    background-color: var(--bg) !important;
    overflow: hidden;
    text-rendering: optimizeLegibility !important;
  }

  main {
    width: 100vw;
    height: 100vh;

    .contents {
      width: 100vw;
      height: 100vh;

      display: flex;
      flex-direction: column;
    }

    .body {
      width: 100%;
      height: 100%;
      display: flex;
    }
  }
</style>
