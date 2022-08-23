<script lang="ts">
  import type Msg from "@core/messages/Msg";
  import { onMount } from "svelte";
  import Message from "./Message.svelte";

  export let chat: Msg[] = [];

  $: chat, scrollToBottom();

  let element: HTMLOListElement;

  onMount(() => {
    scrollToBottom();
  });

  function scrollToBottom() {
    if (element) {
      setTimeout(() => {
        element.scrollTo({ top: element.scrollHeight });
        element.scrollTop = element.scrollHeight;
      }, 100);
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

<ol class="feed" bind:this={element}>
  <div class="header" />
  {#each chat as msg, i}

    <Message message={msg} isHeader={isFirst(i)} />

  {/each}
</ol>

<style lang="scss">

.feed {
  width: 100%;
  height: calc(100vh - 172px - 16px);

  display: block;
  margin: 0;

  font-family: Inter,-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif,"Apple Color Emoji","Segoe UI Emoji"; 
  overflow-y: auto;
  overflow-x: hidden;

  padding-left: 0;
  padding-bottom: 16px;
  list-style: none;

  scroll-behavior: smooth;

  .header {
    width: 100%;
    height: calc(100vh - 166px - 70px);
  }

  &::-webkit-scrollbar {
    width: 0px;
  }
}

</style>