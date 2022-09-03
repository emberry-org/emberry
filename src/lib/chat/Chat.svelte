<script lang="ts">
  import type { Msg, ThreadMsg } from "@core/messages/Msg";
  import { onMount } from "svelte";
  import MessageItem from "./MessageItem.svelte";
  import SmartList from "./SmartList.svelte";

  let items: Array<Msg> = [
    {
      sender: "Mjex",
      content: "@a Gaming?",
      time: "Today at 21:28",
      chain: false,
    },
    {
      sender: "Mjex",
      content: "Let's play some Noita, if you guys\nare up for it?",
      time: "Today at 21:29",
      chain: true,
    },
    <ThreadMsg> {
      sender: "Roboolet",
      content: "Let's go!",
      time: "Today at 21:32",
      messages: [],
      chain: true,
    },
    {
      sender: "Devensiv",
      content: "Sure let's go!",
      time: "Today at 21:34",
      chain: false,
      target: {
        sender: "Mjex",
        content: "@a Gaming?",
        time: "Today at 21:28",
        chain: false,
      },
      remojis: [{ charCode: '😎', quantity: 1 }, { charCode: '👍️', quantity: 3 }],
    }
  ];

  // function getTime(): String {
  //   const now = new Date();
  //   const current =
  //     (now.getHours() < 10 ? "0" + now.getHours() : now.getHours()) +
  //     ":" +
  //     (now.getMinutes() < 10 ? "0" + now.getMinutes() : now.getMinutes());
  //   return current;
  // }

  onMount(() => {
    // for (let i = 0; i < 50; i++) {
    //   items.push({
    //     sender: "Sender",
    //     content: "Content",
    //     time: "Today at " + i,
    //     chain: true,
    //   });
    // }
  });

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