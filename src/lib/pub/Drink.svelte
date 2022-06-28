<script lang="ts">
  //import { getNotificationIcon, getNotificationStyle, Notification } from "@core/messages/Notification";
  import type { Notification } from "@core/messages/Notification";
  //import Icon from "@lib/Icon.svelte";

  export let notification: Notification;

</script>

<div class="drink">
  <!-- <div class="foam" style=" { getNotificationStyle(notification.type) } ">
    <Icon name={ getNotificationIcon(notification.type) } />
  </div> -->
</div>

<style lang="scss">

.drink {
  width: 34px;
  height: 34px;

  margin: 0 6px 0 6px;

  //background-color: #ddd;
  //background-image: url('https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=48');
  background-size: contain;
  border-radius: 8px;

  position: relative;
  pointer-events: all;

  &::after {
    content: "";
    
    width: 30px;
    height: 30px;

    position: absolute;
    top: 0;
    left: 0;

    border: 2px solid #ffffff10;
    border-radius: 8px;
  }

  &:nth-child(1) {
    background-image: url('https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=48');
  }

  &:nth-child(2) {
    background-image: url('https://cdn.discordapp.com/avatars/323181630070063114/22665cff3a55bef4a5bcb3f555589321.webp?size=48');
  }

  &:nth-child(3) {
    background-image: url('https://cdn.discordapp.com/avatars/253216176778248192/2fe0c93cc3361114d268e7e16acc7c97.webp?size=48');
  }

  &:nth-child(4) {
    background-image: url('https://cdn.discordapp.com/avatars/414175361522204672/ffd6f9efede7dba824e4adf3f6916621.webp?size=48');
  }

  // .foam {
  //   position: absolute;

  //   width: 16px;
  //   height: 16px;

  //   top: -4px;
  //   right: -4px;

  //   background-color: #479b4a;
  //   border-radius: 4px !important;

  //   border: 2px solid #222;

  //   box-shadow: 0 0 0 2px #479b4a;

  //   :global(svg) {
  //     position: absolute;

  //     width: 12px;
  //     height: 12px;

  //     color: #fff;
  //     stroke-width: 1px;
  //     stroke: black;

  //     top: 50%;
  //     left: 50%;
  //     transform: translate(-50%, -50%);
  //   }
  // }
}

</style>