<script lang="ts">
  import type Msg from "@core/messages/Msg";

  export let message: Msg;
  export let isHeader: boolean;
</script>

<li class="message">
  {#if isHeader}
    <!-- If this item needs an avatar and username -->

    <div class="head-item">
      <div class="avatar" />

      <div class="details">
        <h3 class="user">
          <span class="username">{message.sender}</span>
          <span class="separator">——</span>
          <span class="timestamp">Today at {message.time ?? "--:--"}</span>
        </h3>
        <div class="contents">{message.content}</div>
      </div>
    </div>
  {:else}
    <!-- If this item is from the same user as the previous one -->

    <div class="body-item">
      <div class="details">
        <div class="contents">{message.content}</div>
      </div>
    </div>
  {/if}
</li>

<style lang="scss">
  .message {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica,
      Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";

    margin: 0;
    padding: 0;
    border: 0;
    vertical-align: baseline;

    &:hover {
      background-color: #ffffff08;
    }

    .head-item {
      min-height: 44px;
      line-height: 16px;

      margin: 17px 0 0 0;
      padding: 4px 48px 1px 16px;

      display: flex;

      .avatar {
        width: 30px;
        min-width: 30px;
        height: 30px;
        margin: 6px 5px 5px 5px;

        background-color: #222222;
        border-radius: 8px;

        background-image: url("../../assets/crabby.png");
        background-size: cover;
        background-repeat: no-repeat;

        position: relative;

        &::after {
          content: "";
          
          width: 24px;
          height: 24px;

          position: absolute;
          top: 0;
          left: 0;

          border: 3px solid #ffffff20;
          border-radius: 8px;
        }
      }

      .details {
        width: fit-content;
        height: fit-content;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: space-between;

        position: static;
        margin-left: 16px;

        .user {
          display: block;
          position: relative;

          line-height: 20px;
          min-height: 20px;
          height: 20px;
          margin: -3px 0 0 0;
          padding: 0;

          user-select: none;
          -webkit-user-select: none;

          .username {
            font-size: 0.85rem;
            font-weight: 500;
            color: #888;
            line-height: 20px;
          }

          .separator {
            font-size: 1rem;
            font-weight: 500;
            color: #444;
            margin: 0 0 0 6px;
            
            line-height: 20px;
          }

          .timestamp {
            font-size: 0.75rem;
            margin: 0 0 0 4px;
            font-weight: 500;
            color: #777;
            line-height: 20px;
          }
        }

        .contents {
          height: fit-content;
          max-width: calc(100vw - 350px);
          inline-size: 100%;
          margin-top: 3px;
          line-height: 20px;
          font-size: 1rem;
          font-weight: 400;
          color: #bbb;
          overflow-wrap: break-word;
          word-break: keep-all;
          white-space: pre-wrap;
        }
      }
    }

    .body-item {
      .details {
        width: fit-content;
        height: fit-content;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: space-between;

        position: static;
        padding-left: 72px;
        padding-bottom: 3px;

        .contents {
          height: fit-content;
          max-width: calc(100vw - 350px);
          inline-size: 100%;
          line-height: 22px;
          font-size: 1rem;
          font-weight: 400;
          color: #bbb;
          overflow-wrap: break-word;
          word-break: keep-all;
          white-space: pre-wrap;
        }
      }
    }
  }
</style>
