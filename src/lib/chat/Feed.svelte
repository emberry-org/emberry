<script lang="ts">
  import type Msg from "@core/messages/Msg";

  export let chat: Msg[] = [];

  /**
 * Returns if the message is from a different user then the one before it.
 */
  const isFirst = (index: number): boolean => {
    if (index == 0) {
      return true; // True if this is the first message in the chat.
    } else if (chat[index].sender != chat[index - 1].sender) {
      return true; // True if the message before is from another sender.
    }
    return false;
  }

</script>

<div class="feed">
  {#each chat as msg, i}
    <div class="item { isFirst(i) ? 'first' : '' }" style="{ `--content:${'\'00:00\''}` }">

      <div class="user">
        <div class="avatar" />
        <div class="username">
          {msg.sender}
        </div>
      </div>

      <div class="content">{msg.content}</div>

    </div>
  {/each}
</div>

<style lang="scss">

.feed {
  width: calc(100% - 32px);
  height: calc(100% - 32px);

  display: flex;
  flex-direction: column;
  justify-content: flex-end;

  padding: 16px;

  font-family: Inter;
  overflow-y: auto;

  .item {
    width: 100%;
    height: 20px;
    margin-top: 4px;
    color: #444444;
    display: flex;
    flex-direction: row;
    align-items: center;
    z-index: 1;

    .user {
      display: flex;
      flex-direction: row;
      align-items: center;
      width: fit-content;
      height: 20px;

      .avatar {
        width: 20px;
        height: 20px;
        border: 1.5px solid #ffffff22;
        background-color: #181a19;
        border-radius: 4px;
      }

      .username {
        width: fit-content;
        min-width: 50px;
        margin-left: 5px;
        margin-right: 10px;
        height: 10px;
        display: flex;
        align-items: center;
        font-size: 16px;
        
        color: #999;
      }

      opacity: 0;
      user-select: none;
    }

    &.first {
      margin-top: 20px;
      position: relative;
      
      .user {
        opacity: 1;
        user-select: all;
      }

      &::before {
        content: " ";
        position: absolute;
        top: -11px;
        right: 0;
        width: calc(100% - 30px);
        height: 1px;
        border-bottom: 1.5px solid #ffffff11;
      }

      &::after {
        content: var(--content);
        position: absolute;
        top: -11px;
        left: 0;
        width: 100%;
        height: 1px;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        font-size: 10px;
        color: #888;
      }
    }

    .content {
      flex-grow: 1;
      height: 10px;
      display: flex;
      align-items: center;
      font-size: 16px;
      color: #ccc;
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