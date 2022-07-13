<script lang="ts">
  import { NotificationType } from "@core/messages/Notification";
  import { PeerStatus } from "@core/PeerStatus";
  import Drink from "@lib/pub/Drink.svelte";
  import { onMount } from "svelte";

  let drinks = [
    { type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } }
  ];

  onMount(() => {
    // setInterval(() => {
    //   drinks.push({ type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } });
    //   drinks = [...drinks];
    // }, 2000);
  });

</script>

<div class="pub">
  <div class="bar">
    {#each drinks as drink}
      <Drink notification={drink} />
    {/each}
    <!-- <Drink notification={{ type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.FriendRequest, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.UnreadMessage, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.IdleChat, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.UnreadPing, user: { id: '1234', status: PeerStatus.Online } }} /> -->
  </div>
</div>

<style lang="scss">
  .pub {
    flex-grow: 1;
    max-width: calc(100vw - 512px);
    height: 100%;
    pointer-events: none;

    .bar {
      width: 100%;
      height: 100%;

      display: flex;
      align-items: center;
      padding: 0 0 0 4px;
    }
  }
</style>
