<script lang="ts">
  import Icon from "./Icon.svelte";
  import { addressBookState } from "../store";

</script>

<div class="address-book" style="margin-left: { $addressBookState ? '0px' : '-300px' }">
  <div class="toolbar">
    <button class="icon-button">
      <Icon name="search" size="16px" />
    </button>
    <button class="icon-button float-right">
      <Icon name="add" size="16px" />
    </button>
  </div>

  <div class="address">
    Mjex 
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address">
    Devensiv 
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address">
    Roboolet 
    <sup>(abcd-1234-efgh-5678)</sup>
  </div>
  <div class="address">
    Funky 
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

    width: fit-content;
    min-width: 128px;
    height: 25px;

    margin-top: 6px;

    border-radius: 4px;
    background-color: #292b2f;
    border: 1.5px solid #ffffff11;
    box-shadow: 0 1px 2px 0 #00000055;
    
    color: #bbb;
    margin-left: 6px;
    padding-left: 3px;
    font-size: 0.7rem;
    font-family: Inter;
    margin-bottom: 1px;
    user-select: none;

    sup {
      position: absolute;

      bottom: 1px;
      right: 3px;

      font-size: 0.5rem;
      font-style: italic;
      color: #888;
    }
  }
}

</style>