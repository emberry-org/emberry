<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import { open } from '@tauri-apps/api/dialog';
  import { readBinaryFile } from '@tauri-apps/api/fs';
  import Icon from "@lib/Icon.svelte";
  import { getProfilePicture, getUsername, onProfilePictureChanged, onUsernameChanged, oppSys, setProfilePicture, setUsername } from '@store';
  import { onMount } from 'svelte';
  import Pub from './Pub.svelte';
  import Modal from '@lib/generic/modal/Modal.svelte';

  $: maximized = false;
  $: hideDecorations = $oppSys == 'linux' || $oppSys == 'darwin';
  $: profileImage = 'background-image: url(data:image/png;base64,' + profilePicture + ')';

  let username = '';
  let profilePicture = '';

  onMount(() => {
    // Get the username from storage.
    username = getUsername();
    if (!username) username = 'NoName';
    usernameDummy = username;
    // Setup the on username changed event.
    onUsernameChanged((newname) => { username = newname; usernameDummy = newname; });

    // Get the profile picture from storage.
    profilePicture = getProfilePicture();
    if (!profilePicture) profilePicture = '';
    // Setup the on profile picture changed event.
    onProfilePictureChanged((newimg) => { profilePicture = newimg });
  });

  appWindow.listen('tauri://resize', async () => {
    maximized = await appWindow.isMaximized();
  });

  function minimize() { appWindow.minimize(); }
  function maximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }

  // Handle the editing of the profile :
  let isEditing = false;

  // Username edit behaviour.
  let usernameInputBox: HTMLInputElement;
  let usernameDummy: string = '';
  const editUsername = () => {
    usernameInputBox.focus();
    usernameInputBox.setSelectionRange(0, usernameInputBox.value.length);
    usernameDummy = username;
    isEditing = true;
  };
  const applyUsername = () => {
    username = usernameDummy;
    isEditing = false;
    usernameInputBox.blur();
    setUsername(usernameDummy);
  };
  const revertUsername = () => {
    isEditing = false;
    usernameInputBox.blur();
    usernameDummy = username;
  };

  // Profile picture edit behaviour.
  const uploadPicture = async () => {
    const path = await open({
      multiple: false,
      filters: [{
        name: 'Image',
        extensions: ['png', 'jpeg', 'webp']
      }]
    });

    if (typeof(path) == 'string') {
      const binary = await readBinaryFile(path);
      const base64 = window.btoa(String.fromCharCode(...new Uint8Array(binary)));

      console.warn('TODO: resize images to avoid exceeding maximum stack size');

      // Update the profile picture in the local store.
      setProfilePicture(base64);
    }
  };

  // Generic keydown event for input boxes.
  const onKeydown = (e: KeyboardEvent) => {
    if (e.key == 'Enter') {
      applyUsername();
    }
  };

</script>

<div class="titlebar" style="--maximized: { maximized || hideDecorations ? '0px' : '1px' }" hidden={hideDecorations}>
  
  <div class="left" data-tauri-drag-region={!hideDecorations || null}>
    <!-- Makes sure you can always drag the window -->
    {#if !hideDecorations} <div class="drag-square"> <Icon name="app/logo" size="24px" /> </div> {/if}

    <div class="profile { maximized || hideDecorations ? 'ml' : '' }">
      <Modal orientation="se" margins="6px 0 0 5px" arrow="false">
        <div class="img" slot="btn" style="{ profileImage }">
        
        </div>

        <div class="details" slot="mdl">
          <div class="media">
            <div class="banner" />
            <div class="picture" on:click={uploadPicture} style="{ profileImage }">
              <Icon name="file/upload" size="32px" />
            </div>
          </div>

          <div class="info">
            <div class="username">

              <input class="{ !isEditing ? 'preview' : 'editable' }" type="text" name="username" 
                bind:this={usernameInputBox} bind:value={usernameDummy} on:keydown={onKeydown}>

              {#if isEditing}
                <button class="btn float-right" on:click={revertUsername}>
                  Revert
                </button>
                <button class="btn apply" on:click={applyUsername}>
                  Apply
                </button>
              {:else}
                <button class="btn float-right" on:click={editUsername}>
                  Edit
                </button>
              {/if}
              
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
      max-width: calc(100% - 44px);
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
          width: fit-content;
          max-width: 150px;
          margin: 0;
          font-weight: normal;
          font-size: 0.9em;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
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

        cursor: auto;

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

            background-color: #1f2022;
            background-size: contain;
            border-radius: 8px;

            display: flex;
            justify-content: center;
            align-items: center;

            cursor: pointer;

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
              transition: background-color 0.1s;
            }

            &:hover {
              &::after {
                background-color: #0004;
              }

              :global(svg) {
                opacity: 1;
              }
            }

            :global(svg) {
              z-index: 1;
              color: #ddd;
              opacity: 0;

              transition: opacity 0.1s;
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
            align-items: center;
            position: relative;

            &::before {
              content: "username";

              position: absolute;
              top: 4px;
              left: 0;

              font-size: 12px;
              font-weight: bold;
              font-variant: small-caps;
              color: #777;

              margin-left: 12px;
            }

            input {
              flex-grow: 1;
              font-size: 14px;
              font-weight: normal;

              margin-left: 6px;
              margin-right: 6px;
              padding-top: 12px;
              padding-left: 6px;

              background-color: transparent;
              border: none;
              border-radius: 4px;
              color: #ccc;
              outline: none;

              &.preview {
                pointer-events: none;
              }

              &.editable {
                background-color: #1f2022;
              }
            }

            .btn {
              width: 64px;
              height: 32px;

              margin-right: 8px;
              background-color: #ffffff18;
              border: 2px solid #ffffff18;
              color: #ccc;

              border-radius: 4px;
              font-size: 14px;

              cursor: pointer;

              &.apply {
                background-color: #125488;
                border: 2px solid #176bae;
                color: #f2eee8;

                &:hover {
                  background-color: #226498;
                  border: 2px solid #277bbe;
                }
              }

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