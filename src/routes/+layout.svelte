<script lang="ts">
  import "../app.css";
  import { goto } from '$app/navigation';
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import Users from "../lib/users.svelte";
  import { onMount } from "svelte";

  let chat: string | undefined = undefined;

  onMount(() => {
    // TODO: check if already connected to tls
    invoke('connect');

    //goto('/chat/' + 'ldslfjsljfsljlejrlqwjelwqjeqlkjeskawkrospaeosafdksaldsajdjsaldss');

    listen("new-room", (e: any) => {
      chat = e.payload;
      console.log('new room : ' + chat);

      goto('/chat/' + chat);
    });
  });
</script>


<main class="app">
  
  <!-- Application Titlebar -->
  <nav></nav>

  <!-- Application -->
  <section>

    <nav class="navbar"></nav>

    <!-- Application Left Bar -->
    <div class="sidebar">

      <Users users={[]} />

    </div>

    <!-- Application Body -->
    <div class="body">

      <slot />

    </div>
    

    <!-- Application Right Bar -->
    <div class="sidebar">

      <nav class="header"></nav>

      <section class="list"></section>

    </div>

  </section>

  <!-- Application Status Bar -->
  <footer></footer>

</main>


<style lang="scss" global>
  html, body, main {
    width: 100vw;
    height: 100vh;

    padding: 0 !important;
    margin: 0 !important;
  }

  .app {
    width: 100vw;
    height: 100vh;

    display: flex;
    flex-direction: column;

    background-color: #1E1E1E;

    > nav {
      width: 100%;
      height: 12px;
    }

    > section {
      flex-grow: 1;

      display: flex;

      .navbar {
        height: 100%;
        width: 72px;
        min-width: 72px;
      }

      .sidebar:nth-child(2) {
        background-color: #2C2C2C;
      }

      .sidebar:nth-child(4) {
        margin-right: 22px;
      }

      .sidebar {
        height: 100%;
        width: 200px;
        min-width: 280px;

        display: flex;
        flex-direction: column;
        border-radius: 12px;

        .header {
          width: 100%;
          height: 160px;
        }

        .list {
          width: 100%;
          flex-grow: 1;

          margin-top: 12px;
          border-radius: 12px;

          background-color: #2C2C2C;
        }
      }

      .body {
        min-width: 0;
        height: 100%;
        flex-grow: 1;

        display: flex;
        flex-direction: column;

        margin: 0 12px;

        .header {
          height: 200px;

          border-radius: 12px;

          background-color: #242424;
        }

        .chat {
          flex-grow: 1;

          margin-top: 12px;
          border-radius: 12px;

          background-color: #383838;
        }
      }
    }

    > footer {
      width: 100%;
      height: 22px;
    }
  }

  .col {
    display: flex;
    flex-direction: column;
  }
</style>