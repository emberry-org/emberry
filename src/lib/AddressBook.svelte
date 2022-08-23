<script lang="ts">
  import Icon from "@icon";
  import { addressBookState } from "@store";

</script>

<div class="address-book" style="margin-left: { $addressBookState ? '0px' : '-300px' }">
  <div class="toolbar">
    <button class="icon-button">
      <Icon name="search" size="14px" />
    </button>
    <button class="icon-button float-right">
      <Icon name="add" size="14px" />
    </button>
  </div>

  <div class="address status-online">
    <Icon name="status" size="64px" />
    <span class="name">Mjex</span>
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address status-connected">
    <Icon name="status" size="64px" />
    <span class="name">Devensiv</span>
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address status-offline">
    <Icon name="status" size="64px" />
    <span class="name">Roboolet</span>
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address status-offline">
    <Icon name="status" size="64px" />
    <span class="name">Funky</span>
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
</div>

<style lang="scss">

.address-book {
  width: 300px;
  height: 100%;

  display: flex;
  flex-direction: column;
  align-items: flex-start;

  background-color: #212327;
  border-right: 1.5px solid #434547;
  transition: margin-left 0.5s ease;

  .toolbar {
    width: 100%;
    height: 32px;

    display: flex;
    align-items: center;

    background-color: #292b2f;
    border-bottom: 1.5px solid #ffffff11;
    box-shadow: 0 1px 2px 0 #00000055;

    button {
      width: 26px;
      margin-left: 4px;
      margin-right: 4px;
    }

    .float-right {
      margin-left: auto;
    }
  }

  .address {
    position: relative;

    width: calc(100% - 22px);
    min-width: 128px;
    height: 30px;

    display: flex;
    align-items: center;
    margin-top: 6px;

    border-radius: 4px;
    background-color: #292b2f;
    border: 1.5px solid #ffffff11;
    box-shadow: 0 1px 2px 0 #00000055;
    
    margin-left: 6px;
    padding-left: 8px;
    user-select: none;
    overflow: hidden;

    :global(svg) {
      position: absolute;
      top: -32px;
      left: -40px;
      opacity: .2;
    }

    .name {
      color: #aaa;
      font-size: 14px;
      font-family: Inter;
      margin-left: 8px;
      z-index: 1;
    }

    sup {
      position: absolute;

      bottom: 1px;
      right: 3px;

      font-size: 0.5rem;
      font-style: italic;
      color: #888;
    }

    &.status {
      &-online {
        color: #50de59;
      }

      &-offline {
        color: #57595c;

        .name {
          color: #777;
        }
      }

      &-connected {
        color: #b175eb;
      }
    }
  }
}

</style>