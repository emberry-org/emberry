<script lang="ts">
  import Icon from "@icon";


</script>

<div class="user-list">

  <div class="user">
    <div class="profile-picture">
      <svg width="0" height="0">
        <defs>
          <clipPath id="picture-clip">
            <rect x="0" y="0" width="18" height="60" fill="#000" />
            <rect x="0" y="0" width="60" height="20" fill="#000" />
          </clipPath>
        </defs>
      </svg>
    </div>
    <div class="profile-picure-status">
      <Icon name="status/pc" size="12px" />
    </div>
    <div class="details">
      <div class="info">
        <p class="username">Username</p>
        <span class="last-active">Today at 00:00</span>
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
    height: 42px;

    border-radius: 8px;

    display: flex;
    align-items: center;
    margin-bottom: 3px;
    position: relative;

    cursor: pointer;

    &:hover {
      background-color: var(--fg);
    }

    .profile-picture {
      margin: 0 6px;
      display: flex;

      -webkit-clip-path: url(#picture-clip);
      clip-path: url(#picture-clip);
    }

    .profile-picure-status {
      position: absolute;

      bottom: 9px;
      left: 26px;

      width: 12px;
      height: 12px;

      color: #8e8;
    }

    .details {
      flex-grow: 1;
      height: 42px;

      .info {
        width: 100%;
        height: 50%;

        display: flex;

        .username {
          font-size: 15px;
          margin-top: 2px;
        }

        .last-active {
          font-size: 11px;
          margin: 6px 12px 0 auto;
          color: #888;
        }
      }

      .status {
        width: 100%;
        height: 50%;

        color: #888;
        font-size: 12px;
      }
    }
  }
}

</style>