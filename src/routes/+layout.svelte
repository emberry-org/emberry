<script lang="ts">
  import "../app.css";
  import loadBundle from "../lib/icons/bundle";
  import { goto } from '$app/navigation';
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from "@tauri-apps/api/event";
  import StatusBar from "../lib/layout/statusbar.svelte";
  import Users from "../lib/users.svelte";
  import Me from "../lib/user.me.svelte";
  import { onMount } from "svelte";

  let chat: string | undefined = undefined;

  onMount(() => {
    loadBundle();

    // TODO: check if already connected to tls
    invoke('connect').catch((e) => {
      if (e === "Already connected to the server") {
        emit("rz-con");
      } else {
        emit("rz-f");
      }
    });

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

      <nav class="header"> <Me /> </nav>

      <section class="list"></section>

    </div>

  </section>

  <!-- Application Status Bar -->
  <StatusBar />

</main>


<style lang="scss" global>
  html, body, main {
    width: 100vw;
    height: 100vh;

    padding: 0 !important;
    margin: 0 !important;
    overflow: hidden;
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
        width: 240px;
        min-width: 240px;

        display: flex;
        flex-direction: column;
        border-radius: 12px;

        .header {
          width: 100%;
          height: fit-content;
          margin-bottom: 12px;
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
          height: 160px;

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
  }

  .col {
    display: flex;
    flex-direction: column;
  }
</style>

