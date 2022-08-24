<script lang="ts">
  import Icon from "@icon";
  import ProfileDetails from "@lib/ProfileDetails.svelte";
  import Modal from '@lib/generic/modal/Modal.svelte';
  import { onMount } from "svelte";
  import { getProfilePicture, navigateTo, onProfilePictureChanged } from "@store";

  $: profileImage = 'background-image: url(data:image/png;base64,' + profilePicture + ')';

  let profilePicture = '';

  onMount(() => {
    // Get the profile picture from storage.
    profilePicture = getProfilePicture();
    if (!profilePicture) profilePicture = '';
    // Setup the on profile picture changed event.
    onProfilePictureChanged((newimg) => { profilePicture = newimg });
  });

</script>

<div class="navbar">
  <div class="item" on:click={() => navigateTo('/')}>
    <Icon name="navigation/home" size="26px" />
  </div>

  <div class="item top-auto">
    <Icon name="navigation/settings" size="26px" />
  </div>
  <div class="item profile">
    <Modal orientation="ne" margins="0 0 -30px 72px" arrow="false">
      <div class="profile-picture md" slot="btn" style="{ profileImage }" />

      <ProfileDetails slot="mdl" />
    </Modal>
  </div>
</div>

<style lang="scss">

.navbar {
  width: 72px;
  height: calc(100% - 16px);
  min-width: 72px;

  display: flex;
  flex-direction: column;
  align-items: center;

  background-color: var(--bg);
  padding: 2px 0 12px 0;

  .item {
    width: 42px;
    height: 42px;

    margin-top: 14px;

    display: flex;
    justify-content: center;
    align-items: center;

    color: #999;
    background-color: var(--mg);
    border-radius: 8px;
    cursor: pointer;

    &:hover {
      background-color: var(--fg);
      color: #ddd;
    }

    &.top-auto {
      margin-top: auto;
    }

    &.profile {
      background-color: transparent !important;

      .profile-picture {
        cursor: pointer;
        width: 42px;
        height: 42px;
      }

      .profile-picture:hover {
        &::after {
          background-color: #fff1;
        }
      }
    }
  }
}

</style>