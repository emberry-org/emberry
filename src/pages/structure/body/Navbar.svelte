<script lang="ts">
  import Icon from "@lib/Icon.svelte";
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
  <div class="item logo">
    <Icon name="app/logo" size="28px" />
  </div>
  <div class="item" on:click={() => navigateTo('/')}>
    <Icon name="navigation/home" size="24px" />
  </div>

  <div class="item top-auto">
    <Icon name="navigation/settings" size="24px" />
  </div>
  <div class="item profile">
    <Modal orientation="ne" margins="0 0 -30px 72px" arrow="false">
      <div class="profile-picture" slot="btn" style="{ profileImage }" />

      <ProfileDetails slot="mdl" />
    </Modal>
  </div>
</div>

<style lang="scss">

.navbar {
  width: 72px;
  height: 100%;
  min-width: 72px;

  display: flex;
  flex-direction: column;
  align-items: center;

  background-color: var(--bg);

  .item {
    width: 52px;
    height: 52px;

    margin-top: 4px;

    display: flex;
    justify-content: center;
    align-items: center;

    color: #888;
    border-radius: 8px;

    &:hover {
      background-color: #ffffff0b;
      color: #ccc;
    }

    &.top-auto {
      margin-top: auto;
    }

    &.logo {
      background-color: transparent !important;
      color: #fff2;
      margin-top: 1px;
    }

    &.profile {
      background-color: transparent !important;
      margin-bottom: 12px;

      .profile-picture {
        cursor: pointer;
        margin-left: 9px;
        margin-top: 9px;
      }
    }
  }
}

</style>