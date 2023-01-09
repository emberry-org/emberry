<script lang="ts">
  import "../app.css";
  import loadBundle from "lib/icons/bundle";
  import { goto } from '$app/navigation';
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from "@tauri-apps/api/event";
  import NavBar from "lib/layout/navbar.svelte";
  import StatusBar from "lib/layout/statusbar.svelte";
  import Users from "lib/users.svelte";
  import Me from "lib/user/user.me.svelte";
  import { onMount } from "svelte";
  import Cmd from "lib/cmd/cmd.svelte";
  import { setItem } from "lib/store";
  import { UserStatus } from "lib/user";

  onMount(() => {
    loadBundle();

    invoke('connect').catch((e) => {
      if (e === "Already connected to the server") {
        emit("rz-con");
      } else {
        console.error(e);
        emit("rz-f");
      }
    });

    // TODO: listen for failed room requests and mark those users as Offline

    listen("new-room", (e: any) => {
      const room_id = e.payload.room_id;
      const peer_id = e.payload.peer_id;
      setItem(peer_id, JSON.stringify(UserStatus.Connected));

      console.log(`new room : ${peer_id}:${room_id}`);
      goto(`/chat/${peer_id}:${room_id}`);
    });
  });
</script>


<main class="app">

  <Cmd />

  <aside>
    <NavBar />
  </aside>

  <div class="page">
    <div class="content">
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
    </div>

    <!-- Application Status Bar -->
    <StatusBar />
  </div>

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
    flex-direction: row;

    background-color: var(--bg-200);

    > aside {
      height: 100vh;
      display: flex;
    }

    .page {
      flex-grow: 1;
      display: flex;
      margin-top: 20px;
      flex-direction: column;

      .content {
        flex-grow: 1;
        margin-right: 20px;
        display: flex;
        flex-direction: row;

        .navbar {
          height: 100%;
          width: 72px;
          min-width: 72px;
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

            background-color: var(--bg-100);
          }
        }

        .body {
          min-width: 0;
          height: 100%;
          flex-grow: 1;

          display: flex;
          flex-direction: column;

          margin: 0 20px;

          .header {
            height: 100px;

            border-radius: 12px;

            background-color: var(--bg-300);
          }

          .chat {
            flex-grow: 1;

            margin-top: 12px;
            border-radius: 12px;

            background-color: var(--bg-300);
          }
        }
      }
    }
  }

  .col {
    display: flex;
    flex-direction: column;
  }
</style>

