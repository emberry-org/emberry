<script lang="ts">
  import type Msg from "@core/messages/Msg";

  export let chat: Msg[] = [];

  /**
 * Returns if the message is from a different user then the one before it.
 */
  function isFirst(index: number): boolean {
    if (index == 0 || chat[index].sender != chat[index - 1].sender) {
      // True if this is the first message in the chat.
      // True if the message before is from another sender.
      return true; 
    }
    return false;
  }

</script>

<div class="feed">
  {#each chat as msg, i}
    <div class="item { isFirst(i) ? '' : 'no-decorations' }" style="{ `--content:${'\'00:00\''}` }">

      <div class="avatar" />

      <div class="content">
        <div class="user">
          <div class="username">{msg.sender}</div>
          <div class="dot">·</div>
          <div class="timestamp">11:34</div>
        </div>
        <div class="message">{msg.content}</div>
      </div>

    </div>
  {/each}
</div>

<style lang="scss">

.feed {
  width: calc(100% - 32px);
  height: calc(100vh - 175px);

  display: flex;
  flex-direction: column;
  justify-content: flex-end;

  padding: 16px;

  font-family: Inter;
  overflow-y: auto;

  .item {
    width: 100%;
    height: 32px;
    margin-top: 20px;
    color: #444444;
    display: flex;
    flex-direction: row;
    align-items: center;
    z-index: 1;

    .avatar {
      width: 32px;
      height: 32px;
      margin-top: 2px;
      border: 1.5px solid #ffffff22;
      background-color: #212327;
      border-radius: 6px;
    }

    .content {
      flex-grow: 1;
      height: 32px;

      display: flex;
      flex-direction: column;
      align-items: flex-start;
      justify-content: space-between;

      .user {
        width: 100%;
        height: 14px;

        display: flex;
        align-items: center;

        .username {
          margin-left: 16px;
          font-size: 14px;
          color: #fff;
          user-select: none;
        }

        .dot {
          margin: 0 8px 0 10px;
          color: #ffffff33;
          user-select: none;
        }

        .timestamp {
          font-size: 10px;
          margin-top: 1px;
          color: #ffffff44;
          user-select: none;
        }
      }

      .message {
        width: 100%;
        height: 14px;
        margin-left: 16px;
        font-size: 14px;
        color: #ddd;
      }
    }

    &.no-decorations {
      height: 14px;
      margin-top: 6px;
      .avatar { opacity: 0; }
      .content .user { display: none; }
      .content { height: 14px; }
    }
  }

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background-color: #00000022;
    border-radius: 100px;
  }

  &::-webkit-scrollbar-thumb {
    border-radius: 100px;
    background: #00000022;
    &:hover {
      background: #ffffff22;
    }
  }
}

</style>