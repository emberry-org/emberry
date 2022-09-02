<script lang="ts">
  import type { Msg, ThreadMsg } from "@core/messages/Msg";
  //import { onMount } from "svelte";
  import MessageItem from "./MessageItem.svelte";
  import SmartList from "./SmartList.svelte";

  let items: Array<Msg> = [
    {
      sender: "Mjex",
      content: "@a Gaming?",
      time: "Today at 21:28",
    },
    {
      sender: "Mjex",
      content: "Let's play some Noita,\nif you guys are up for it?",
      time: "Today at 21:29",
    },
    <ThreadMsg> {
      sender: "Roboolet",
      content: "Let's go!",
      time: "Today at 21:32",
      messages: [],
    },
    {
      sender: "Devensiv",
      content: "Sure let's go!",
      time: "Today at 21:34",
      target: {
        sender: "Mjex",
        content: "@a Gaming?",
        time: "Today at 21:28",
      },
      remojis: [{ charCode: '😎', quantity: 1 }, { charCode: '👍️', quantity: 3 }],
    },
  ];

  // onMount(() => {
  //   for (let i = 0; i < 100; i++) {
  //     items.push({ sender: 'User', content: i + ' Hello world!' });
  //   }
  // });

</script>

<div class="chat">
  <SmartList {items} reverse={true} let:item>
    <MessageItem {...item}/>
  </SmartList>
</div>

<style lang="scss">

  .chat {
    width: 100%;
    height: 100%;
  }

</style>