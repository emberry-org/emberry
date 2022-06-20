<script lang="ts">
  import type Msg from "@core/messages/Msg";
  import { onMount } from "svelte";

  export let chat: Msg[] = [];

  $: chat, scrollToBottom();

  let element: HTMLDivElement;

  onMount(() => {
    element.scrollTop = element.scrollHeight;
  });

  function scrollToBottom() {
    if (element) {
      element.scrollTo({ top: element.scrollHeight });
    }
  }

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

<div class="feed" bind:this={element}>
  {#each chat as msg, i}

    {#if isFirst(i)} <!-- If this item needs an avatar and username -->

      <div class="head-item">

        <div class="avatar" />

        <div class="content">
          <div class="user">
            <div class="username">{ msg.sender }</div>
            <div class="dot">·</div>
            <div class="timestamp">11:34</div>
          </div>
          <div class="message">{ msg.content }</div>
        </div>

      </div>

    {:else} <!-- If this item is from the same user as the previous one -->
      
      <div class="body-item">

        <div class="message">
          { msg.content }
        </div>

      </div>

    {/if}

  {/each}
</div>

<style lang="scss">

.feed {
  width: calc(100% - 32px);
  height: calc(100vh - 175px);

  display: flex;
  flex-direction: column;
  justify-content: flex-start;

  padding: 16px;

  font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  overflow-y: auto;
  overflow-x: hidden;

  .head-item {
    width: 100%;
    height: fit-content;
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
          -webkit-user-select: none;
        }

        .dot {
          margin: 0 8px 0 10px;
          color: #ffffff33;
          user-select: none;
          -webkit-user-select: none;
        }

        .timestamp {
          font-size: 10px;
          margin-top: 1px;
          color: #ffffff44;
          user-select: none;
          -webkit-user-select: none;
        }
      }

      .message {
        inline-size: calc(100% - 42px);
        margin-left: 16px;
        margin-top: 4px;
        font-size: 14px;
        color: #ddd;
        overflow-wrap: break-word;
        word-break: keep-all; 
      }
    }
  }

  .body-item {
    padding-left: 51px;

    .message {
      inline-size: calc(100% - 42px);
      font-size: 14px;
      color: #ddd;
      overflow-wrap: break-word;
      word-break: keep-all; 
    }
  }

  &::-webkit-scrollbar {
    width: 0px;
  }
}

</style>