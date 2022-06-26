<script lang="ts">
  import { NotificationType } from "@core/messages/Notification";
  import { PeerStatus } from "@core/PeerStatus";
  import Drink from "@lib/pub/Drink.svelte";

</script>

<div class="pub">
  <div class="bar">
    <Drink notification={{ type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.FriendRequest, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.UnreadMessage, user: { id: '1234', status: PeerStatus.Online } }} />
    <Drink notification={{ type: NotificationType.UnreadPing, user: { id: '1234', status: PeerStatus.Online } }} />
  </div>
</div>

<style lang="scss">
  .pub {
    flex-grow: 1;
    height: 100%;
    pointer-events: none;

    .bar {
      width: 100%;
      height: 100%;

      display: flex;
      padding: 0 0 0 4px;
    }
  }
</style>
