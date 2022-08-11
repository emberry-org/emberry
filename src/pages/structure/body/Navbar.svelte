<script lang="ts">
  import Icon from "@lib/Icon.svelte";

</script>

<div class="navbar">
  <div class="item logo">
    <Icon name="app/logo" size="28px" />
  </div>
  <div class="item">
    <Icon name="navigation/settings" size="24px" />
  </div>
  <div class="item">
    <Icon name="navigation/settings" size="24px" />
  </div>

  <!-- <div class="item profile">
    <div class="profile-picture" />
  </div> -->
</div>

<style lang="scss">

.navbar {
  width: 66px;
  height: 100%;
  min-width: 66px;

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

    &.logo {
      margin-top: 8px;

      background-color: transparent !important;
      color: #fff2;
    }

    &.profile {
      background-color: transparent !important;
      margin-top: auto;

      .profile-picture {
        cursor: pointer;
      }
    }
  }
}

</style>