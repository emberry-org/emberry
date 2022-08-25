<script lang="ts">

</script>

<div class="user-list">

  <div class="user">
    <div class="profile-picture md" />
    <div class="details">
      <div class="info">
        <p class="username">Username</p>
        <span class="last-active">00:00</span>
      </div>
      <div class="status">Status</div>
    </div>
  </div>

  <div class="user">
    <div class="profile-picture md" />
    <div class="details">
      <div class="info">
        <p class="username">Username</p>
        <span class="last-active">00:00</span>
      </div>
      <div class="status">Status</div>
    </div>
  </div>

  <div class="user">
    <div class="profile-picture md" />
    <div class="details">
      <div class="info">
        <p class="username">Username</p>
        <span class="last-active">00:00</span>
      </div>
      <div class="status">Status</div>
    </div>
  </div>

  <div class="user">
    <div class="profile-picture md" />
    <div class="details">
      <div class="info">
        <p class="username">Username</p>
        <span class="last-active">00:00</span>
      </div>
      <div class="status">Status</div>
    </div>
  </div>

</div>

<style lang="scss">

.user-list {
  width: calc(100% - 16px);
  height: calc(100% - 22px);

  padding: 11px 8px;
  display: flex;
  flex-direction: column;

  .user {
    width: 100%;
    height: 52px;

    border-radius: 8px;

    display: flex;
    align-items: center;
    margin-bottom: 3px;

    cursor: pointer;

    &:hover {
      background-color: var(--fg);
    }

    .profile-picture {
      margin: 0 6px;
    }

    .details {
      flex-grow: 1;
      height: 42px;

      .info {
        width: 100%;
        height: 50%;

        display: flex;

        .username {
          font-size: 16px;
          font-weight: bold;
        }

        .last-active {
          font-size: 11px;
          margin: 6px 12px 0 auto;
          color: #aaa;
        }
      }

      .status {
        width: 100%;
        height: 50%;

        color: #888;
        font-size: 14px;
      }
    }
  }
}

</style>