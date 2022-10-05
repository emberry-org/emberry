<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import Users from "../lib/users.svelte";
  import { onMount } from "svelte";

  let chat: string | undefined = undefined;

  onMount(() => {
    // TODO: check if already connected to tls
    invoke('connect');

    listen("new-room", (e: any) => {
      chat = e.payload;
    });
  });
</script>


<main class="container">
  
  <!-- Application Titlebar -->
  <nav></nav>

  <!-- Application -->
  <section>

    <nav class="navbar"></nav>

    <!-- Application Left Bar -->
    <section class="sidebar">

      <Users users={[]} />

    </section>

    <!-- Application Body -->
    <section class="body">

      <slot />

    </section>
    

    <!-- Application Right Bar -->
    <section class="sidebar">

      <nav class="header"></nav>

      <section class="list"></section>

    </section>

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

  .container {
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
      width: 100%;
      flex-grow: 1;

      display: flex;

      .navbar {
        height: 100%;
        width: 72px;
      }

      .sidebar:nth-child(2) {
        background-color: #2C2C2C;
      }

      .sidebar:nth-child(4) {
        margin-right: 22px;
      }

      .sidebar {
        height: 100%;
        width: 280px;

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
        height: 100%;
        flex-grow: 1;

        display: flex;
        flex-direction: column;

        margin: 0 12px;

        .header {
          width: 100%;
          height: 240px;

          border-radius: 12px;

          background-color: #242424;
        }

        .chat {
          width: 100%;
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