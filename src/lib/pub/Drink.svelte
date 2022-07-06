<script lang="ts">
  import { getNotificationIcon, getNotificationStyle, Notification } from "@core/messages/Notification";
  import Modal from "@lib/generic/modal/Modal.svelte";
  import Icon from "@lib/Icon.svelte";

  export let notification: Notification;

</script>

<div class="drink">

  <Modal>
    <div class="picture-btn" slot="btn">
      <div class="picture">
        <svg width="0" height="0">
          <defs>
            <clipPath id="drink-clip">
              <path
           style="fill:#000000;stroke-width:0.0399999;stroke-linecap:round;stroke-linejoin:round;stroke-opacity:0.07;paint-order:stroke markers fill"
           d="M 0 0 L 0 34 L 15 34 L 15 20 A 4.9999999 4.9999999 0 0 1 20 15 L 34 15 L 34 0 L 0 0 z " />
            </clipPath>
          </defs>
        </svg>
      </div>
    
      <div class="foam" style="{ getNotificationStyle(notification.type) }">
        <Icon name={ getNotificationIcon(notification.type) } size="16px" />
      </div>
    </div>
  
    <div class="details" slot="mdl">
      <header>
        <h3> Username </h3>
        <p> #1234 </p>
        <button class="close-btn" id="close-btn">
          <Icon name="window/close" size="20px" />
        </button>
      </header>
      <div class="body">

      </div>
    </div>
  </Modal>
  
</div>

<style lang="scss">

.drink {
  width: 34px;
  height: 44px;

  margin: 0 6px 0 6px;

  position: relative;
  pointer-events: all;

  .picture-btn {
    margin-top: 5px;
    cursor: pointer;
  }

  &:nth-child(1) .picture {
    background-image: url('https://cdn.discordapp.com/avatars/274954769846501376/ce8cedc7e70deedda89d8b17643e8647.webp?size=48');
  }

  &:nth-child(2) .picture {
    background-image: url('https://cdn.discordapp.com/avatars/323181630070063114/22665cff3a55bef4a5bcb3f555589321.webp?size=48');
  }

  &:nth-child(3) .picture {
    background-image: url('https://cdn.discordapp.com/avatars/253216176778248192/2fe0c93cc3361114d268e7e16acc7c97.webp?size=48');
  }

  &:nth-child(4) .picture {
    background-image: url('https://cdn.discordapp.com/avatars/414175361522204672/ffd6f9efede7dba824e4adf3f6916621.webp?size=48');
  }
  
  .picture {
    width: 34px;
    height: 34px;

    background-size: contain;
    border-radius: 8px;

    -webkit-clip-path: url(#drink-clip);
    clip-path: url(#drink-clip);

    &::after {
      content: "";
      
      width: 30px;
      height: 30px;

      position: absolute;
      top: 0;
      left: 0;

      border: 2px solid #ffffff20;
      border-radius: 8px;
    }
  }

  .foam {
    position: absolute;

    width: 14px;
    height: 14px;

    bottom: 10.5px;
    right: 0px;

    background-color: #479b4a33;
    border-radius: 4px !important;

    box-shadow: 0 0 0 2px #479b4a33;

    :global(svg) {
      position: absolute;

      stroke-width: 2px;
      stroke: black;

      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
    }
  }
}

.details {
  width: 260px;
  height: 128px;

  display: flex;
  flex-direction: column;

  border-radius: 5px;
  overflow: hidden;

  h3 {
    font-size: 14px;
    font-weight: normal;
    margin-left: 16px;
  }

  p {
    font-size: 12px;
    font-weight: normal;
    margin-left: 8px;
    padding-top: 2px;
  }

  header {
    background-color: #262729;
  }

  .body {
    width: 100%;

    background-color: #323335;
  }
}

</style>