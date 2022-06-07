<script lang="ts">
  import Home from "./pages/Home.svelte";
  import CommandCenter from './comps/CommandCenter.svelte';

  let commandCenter: boolean = false;

  document.addEventListener("keydown", function(e) {
    console.log("keydown: ", e);
    if (e.ctrlKey && e.shiftKey && e.code === "KeyP") {
      e.preventDefault();
      commandCenter = !commandCenter;
    }
  });
</script>

<main>
  {#if commandCenter}
    <CommandCenter />
  {/if}
  
  <Home />
</main>

<style lang="scss" global>
  @import './style/fonts.css';
  @import './style/button.scss';

  :root {
    font-family: Josefin, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;

    --background: #1a1a1a;
    --background-border: #242424;
    --middleground: #202020;
    --middleground-border: #292929;
    --foreground: #2b2b2b;
    --foreground-border: #363636;

    --logo: #ffffff;
    --highlight: #ececec;
    --text: #d2c2c3;
    --lowlight: #525252;

    --green: #7ce38b;
    --red: #fa7970;
    --orange: #faa356;
    --blue: #77bdfb;
    --purple: #cea5fb;
  }

  html,
  body {
    padding: 0;
    margin: 0;
  }

  main {
    width: 100vw;
    height: 100vh;

    display: flex;
    flex-direction: column;

    background-color: var(--middleground);
  }
</style>
