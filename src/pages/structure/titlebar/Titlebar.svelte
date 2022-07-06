<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import Icon from "@lib/Icon.svelte";
  import { getUsername, onUsernameChanged, oppSys } from '@store';
  import { onMount } from 'svelte';
  import Pub from './Pub.svelte';
  import Modal from '@lib/generic/modal/Modal.svelte';

  $: maximized = false;
  $: hideDecorations = $oppSys == 'linux' || $oppSys == 'darwin';

  let username = '';

  onMount(() => {
    // Get the username from storage.
    username = getUsername();
    if (!username) username = 'NoName';
    // Setup the on username changed event.
    onUsernameChanged((newname) => { username = newname });
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
    {#if !maximized && !hideDecorations} <div class="drag-square"> <Icon name="app/logo" size="24px" /> </div> {/if}

    <div class="profile { maximized || hideDecorations ? 'ml' : '' }">
      <Modal orientation="se" margins="6px 0 0 0">
        <div class="img" slot="btn" style="background-image: url(https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=48)">
        
        </div>

        <div class="details" slot="mdl">
          <div class="media">
            <div class="banner" />
            <div class="picture" style="background-image: url(https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=128)" />
          </div>

          <div class="info">
            <div class="username">
              <p> { username } </p>
              <button class="edit-btn">
                Edit
              </button>
            </div>
          </div>
        </div>
      </Modal>
      <div class="info">
        <h3> { username } </h3>
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
  height: 44px;
  min-height: 44px;
  
  display: flex;
  align-items: flex-start;
  z-index: 1;

  background-color: #1f2022;

  .left {
    width: 248px;
    min-width: 248px;
    height: 100%;

    display: flex;

    .drag-square {
      width: 44px;
      min-width: 44px;
      height: 44px;

      pointer-events: none;

      display: flex;
      justify-content: center;
      align-items: center;

      color: #fff1;
    }

    .profile {
      width: fit-content;
      height: 100%;

      display: flex;
      align-items: center;

      &.ml {
        margin-left: 6px;
      }

      .img {
        width: 34px;
        height: 34px;

        margin-right: 8px;
        margin-top: 5px;
        position: relative;

        background-size: contain;
        border-radius: 8px;

        cursor: pointer;

        &::after {
          content: "";
          
          width: 30px;
          height: 30px;

          position: absolute;
          top: 0;
          left: 0;

          border: 2px solid #ffffff20;
          border-radius: 8px;
        }
      }

      > .info {
        flex-grow: 1;
        height: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;

        h3 {
          margin: 0;
          font-weight: normal;
          font-size: 0.9em;
        }

        p {
          margin: 0;
          font-size: 0.7em;
        }
      }

      // ========== User Profile Popup ==========
      .details {
        width: 420px;
        height: 202px;

        background-color: #323335;

        border-radius: 5px;
        overflow: hidden;

        display: flex;
        flex-direction: column;

        cursor: default;

        // ========== Profile Media ==========
        .media {
          width: 100%;
          height: 160px;

          background-color: #262729;

          .banner {
            width: 100%;
            height: 120px;

            background-image: url('https://c4.wallpaperflare.com/wallpaper/974/623/684/os-x-el-capitan-mountains-5k-macos-wallpaper-preview.jpg');
            background-size: cover;
            background-position: center bottom;

            box-shadow: inset 0 0 24px 12px #00000088;
          }

          .picture {
            width: 64px;
            height: 64px;

            position: absolute;
            top: 80px;
            left: 50%;

            transform: translateX(-50%);

            background-size: contain;
            border-radius: 8px;

            &::after {
              content: "";
              
              width: 56px;
              height: 56px;

              position: absolute;
              top: 0;
              left: 0;

              border: 4px solid #ffffff20;
              border-radius: 8px;

              box-shadow: 0 0 0 8px #262729;
            }
          }
        }

        // ========== Profile Info ==========
        .info {
          width: 100%;
          flex-grow: 1;

          display: flex;
          flex-direction: column;

          .username {
            width: 100%;
            height: 42px;

            display: flex;
            justify-content: space-between;
            align-items: center;

            p {
              font-size: 14px;
              font-weight: normal;
              margin-left: 12px;
            }

            .edit-btn {
              width: 64px;
              height: 32px;

              margin-right: 8px;

              background-color: #ffffff18;
              border: 2px solid #ffffff18;
              border-radius: 4px;
              color: #ddd;

              font-size: 14px;

              cursor: pointer;

              &:hover {
                background-color: #ffffff28;
                border: 2px solid #ffffff28;
              }
            }
          }
        }
      }
    }
  }

  .right {
    flex-grow: 1;
    height: 100%;

    display: flex;
  }

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