<script lang="ts">
  import { Body, Titlebar } from "@page/structure";
  import CommandCenter from '@lib/window/CommandCenter.svelte';
  import { commandCenterState } from "@store";
  import setupOS from "@core/OppSys";
  import { onMount } from "svelte";
  import { navigate } from "svelte-navigator";

  onMount(() => {
    setupOS();

    //navigate('/chat/1234');
  });

  /** Check for local shortcuts */
  document.addEventListener("keydown", function(e) {
    if (e.ctrlKey && e.shiftKey && e.code === "KeyP") {
      e.preventDefault();
      commandCenterState.set(true);
    }
  });
</script>

<main>
  {#if $commandCenterState}
    <CommandCenter />
  {/if}

  <Titlebar />

  <Body />

</main>

<style lang="scss" global>
  @import './style/fonts.css';
  @import './style/global.scss';

  :root {
    font-family: Amulya, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;

    --bg: #202020;
    --bg-border: #303030;
    --mg: #272727;
    --mg-border: #1d1d1d;
    --fg: #2d2d2d;
    --fg-border: #161616;

    --primary: #60cdff;

    --logo: #ffffff;
    --highlight: #ffffff;
    --text: #cccccc;
    --lowlight: #888888;

    --green: #7ce38b;
    --red: #fa7970;
    --orange: #faa356;
    --blue: #60cdff;
    --purple: #cea5fb;
  }

  html,
  body {
    padding: 0;
    margin: 0;
    background-color: transparent !important;
    overflow: hidden;
  }

  main {
    width: 100vw;
    height: 100vh;

    display: flex;
    flex-direction: column;

    .body {
      width: 100%;
      height: 100%;
      display: flex;
    }
  }
</style>
