<script lang="ts">
  import type Msg from "@core/messages/Msg";
  import { onMount } from "svelte";
import { merge_ssr_styles } from "svelte/internal";

  export let chat: Msg[] = [];

  $: chat, scrollToBottom();

  let element: HTMLDivElement;

  onMount(() => {
    element.scrollTop = element.scrollHeight;
  });

  function scrollToBottom() {
    if (element) {
      setTimeout(() => {
        element.scrollTo({ top: element.scrollHeight });
      }, 200);
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
            <div class="timestamp">{ msg.time ?? '--:--' }</div>
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
  width: 100%;
  height: calc(100vh - 116px);

  display: flex;
  flex-direction: column;
  justify-content: flex-start;

  font-family: Inter,-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji"; 
  overflow-y: auto;
  overflow-x: hidden;

  .head-item {
    width: 100%;
    height: fit-content;
    margin-top: 16px;
    color: #444444;
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    z-index: 1;

    &:hover {
      background-color: #222222;
    }

    .avatar {
      width: 38px;
      min-width: 38px;
      height: 38px;
      
      margin-left: 16px;
      background-color: #222222;
      border-radius: 6px;

      background-image: url("../../assets/secret.jpg");
      background-size: cover;
      background-repeat: no-repeat;
      position: relative;

      &::after {
        content: "";
        position: absolute;

        top: 0;
        left: 0;

        width: 36px;
        height: 36px;

        border: 1px solid #ffffff33;
        border-radius: 6px;
      }
    }

    .content {
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
        line-height: 20px;
        font-size: 14px;
        color: #ddd;
        overflow-wrap: break-word;
        word-break: keep-all; 
      }
    }
  }

  .body-item {
    padding-left: 70px;
    padding-top: 4px;

    &:hover {
      background-color: #222222;
    }

    .message {
      line-height: 20px;
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