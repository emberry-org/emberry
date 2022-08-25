<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { getProfilePicture, getUsername, onProfilePictureChanged, onUsernameChanged, setProfilePicture, setUsername } from "@store";
  import { readBinaryFile } from '@tauri-apps/api/fs';
  import { resizeBase64Image } from '@core/utils/Img';
  import Icon from "@icon";
  import { onMount } from 'svelte';

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
        extensions: ['png', 'jpg', 'jpeg', 'webp']
      }]
    });

    if (typeof(path) == 'string') {
      const binary = await readBinaryFile(path);
      const base64 = window.btoa(new Uint8Array(binary).reduce(function (data, byte) {
        return data + String.fromCharCode(byte);
      }, ''));

      resizeBase64Image(base64, [256, 256], setProfilePicture);
    }
  };

  // Generic keydown event for input boxes.
  const onKeydown = (e: KeyboardEvent) => {
    if (e.key == 'Enter') {
      applyUsername();
    }
  };
</script>

<div class="profile-details">
  <div class="media">
    <div class="banner" />
    <div class="profile-picture lg" on:click={uploadPicture} style="{ profileImage }">
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
        <button class="btn positive" on:click={applyUsername}>
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

<style lang="scss">

.profile-details {
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

    .profile-picture {
      position: absolute;
      top: 80px;
      left: 50%;

      transform: translateX(-50%);

      display: flex;
      justify-content: center;
      align-items: center;

      cursor: pointer;

      &::after {
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
        margin-right: 8px;
      }
    }
  }
}

</style>