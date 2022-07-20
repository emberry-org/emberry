<script lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import type Drink from "@core/Drink";
  import { default as DrinkComp } from "@lib/pub/Drink.svelte";
  import { addDrink, onDrinksChanged } from "@store";
  import { onMount } from "svelte";
  import { NotificationType } from '@core/messages/Notification';
  import { PeerStatus } from '@core/PeerStatus';

  let drinks: Drink[] = [
    // TEMP : Test drink
    { type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } }
  ];

  // const addDrink = (drink: Drink) => {
  //   drinks.push(drink);
  //   drinks = [...drinks]; // Updates the UI.
  // };

  onMount(() => {
    // Emitted when a room request comes in over TLS
    listen('room_request', event => {
      addDrink({ type: NotificationType.ConnectionRequest, user: { id: (event.payload as any).key, status: PeerStatus.Online } });
    });

    onDrinksChanged(newValue => {
      drinks = newValue;
    });
    // setInterval(() => {
    //   drinks.push({ type: NotificationType.ConnectionRequest, user: { id: '1234', status: PeerStatus.Online } });
    //   drinks = [...drinks];
    // }, 2000);
  });

  const drinkActivated = (index: number) => {
    console.log('test: ', index);
    // TODO : Handle incoming drink activations.
  };

</script>

<div class="pub">
  <div class="bar">
    {#each drinks as drink, i}
      <DrinkComp notification={drink} on:activate={() => drinkActivated(i)} />
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
