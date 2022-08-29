<script lang="ts">
  import { onMount } from "svelte";
  import MessageItem from "./MessageItem.svelte";
  import SmartList from "./SmartList.svelte";

  let items = [];

  onMount(() => {
    for (let i = 0; i < 100; i++) {
      items.push({ sender: 'User', content: i + ' Hello world!' });
    }
  });

</script>

<div class="chat">
  <SmartList {items} let:item>
    <MessageItem {...item}/>
  </SmartList>
</div>

<style lang="scss">

  .chat {
    width: 100%;
    height: 100%;
  }

</style>