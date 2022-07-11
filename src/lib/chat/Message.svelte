<script lang="ts">
  import type Msg from "@core/messages/Msg";

  export let message: Msg;
  export let isHeader: boolean;
</script>

<div class="message">
  {#if isHeader}
    <!-- If this item needs an avatar and username -->

    <div class="head-item">
      <div class="avatar" />

      <div class="details">
        <div class="user">
          <div class="username">{message.sender}</div>
          <div class="dot">·</div>
          <div class="timestamp">{message.time ?? "--:--"}</div>
        </div>
        <div class="contents">{message.content}</div>
      </div>
    </div>
  {:else}
    <!-- If this item is from the same user as the previous one -->

    <div class="body-item">
      <div class="contents">
        <span class="timestamp">{message.time ?? "--:--"}</span> <span class="dot">·</span> {message.content}
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .message {
    width: fit-content;
    height: fit-content;

    background-color: #ffffff08;

    margin: 8px 12px 0px 12px;
    padding: 12px;
    border-radius: 8px;

    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica,
      Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";

    &:hover {
      background-color: #ffffff0f;
    }

    .head-item {
      width: 100%;
      height: fit-content;
      color: #444444;
      display: flex;
      flex-direction: row;
      align-items: flex-start;

      .avatar {
        width: 38px;
        min-width: 38px;
        height: 38px;

        background-color: #222222;
        border-radius: 6px;

        background-image: url("../../assets/secret.jpg");
        background-size: cover;
        background-repeat: no-repeat;

        // &::after {
        //   content: "";
        //   position: absolute;

        //   top: 0;
        //   left: 0;

        //   width: 32px;
        //   height: 32px;

        //   border: 3px solid #ffffff28;
        //   border-radius: 6px;
        // }
      }

      .details {
        flex-grow: 1;
        height: fit-content;

        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: space-between;

        .user {
          width: 100%;
          height: 14px;

          display: flex;
          align-items: center;
          margin-top: 2px;

          .username {
            margin-left: 16px;
            font-size: 14px;
            color: #f7f7f7;
            user-select: none;
            -webkit-user-select: none;
          }

          .dot {
            margin: 0 8px 0 10px;
            color: #6e6e6e;
            user-select: none;
            -webkit-user-select: none;
          }

          .timestamp {
            font-size: 10px;
            margin-top: 1px;
            color: #6e6e6e;
            user-select: none;
            -webkit-user-select: none;
          }
        }

        .contents {
          height: fit-content;
          inline-size: calc(100% - 42px);
          margin-left: 16px;
          margin-top: 4px;
          line-height: 20px;
          font-size: 14px;
          color: #ddd;
          overflow-wrap: break-word;
          word-break: keep-all;
          white-space: pre;
        }
      }
    }

    .body-item {
      .contents {
        height: fit-content;
        line-height: 20px;
        inline-size: calc(100% - 42px);
        font-size: 14px;
        color: #ddd;
        overflow-wrap: break-word;
        word-break: keep-all;
        white-space: pre;

        display: flex;
        flex-direction: row;

        .dot {
          margin: 0 8px 0 10px;
          color: #6e6e6e;
          user-select: none;
          -webkit-user-select: none;
        }

        .timestamp {
          font-size: 10px;
          margin-top: 1px;
          color: #6e6e6e;
          user-select: none;
          -webkit-user-select: none;
        }
      }
    }
  }
</style>
