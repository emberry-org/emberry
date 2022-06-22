<script lang="ts">
  import { getNotificationIcon, getNotificationStyle, Notification } from "@core/messages/Notification";
  import Icon from "@lib/Icon.svelte";

  export let notification: Notification;

</script>

<div class="drink">
  <div class="foam" style=" { getNotificationStyle(notification.type) } ">
    <Icon name={ getNotificationIcon(notification.type) } />
  </div>
</div>

<style lang="scss">

.drink {
  width: 36px;
  height: 36px;

  margin: 14px 14px 0 14px;

  background-color: #333333;
  border: 1px solid #545454;
  border-radius: 4px;

  position: relative;

  .foam {
    position: absolute;

    width: 16px;
    height: 16px;

    top: -4px;
    right: -4px;

    background-color: #479b4a;
    border-radius: 4px !important;

    border: 2px solid #222;

    box-shadow: 0 0 0 2px #479b4a;

    :global(svg) {
      position: absolute;

      width: 12px;
      height: 12px;

      color: #fff;
      stroke-width: 1px;
      stroke: black;

      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
    }
  }
}

</style>