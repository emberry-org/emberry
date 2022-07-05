<script lang="ts">
  import Icon from "@lib/Icon.svelte";

</script>

<div class="navbar">
  <div class="item">
    <Icon name="navigation/settings" size="20px" />
  </div>
  <div class="item">
    <Icon name="navigation/settings" size="20px" />
  </div>
  <div class="item">
    <Icon name="navigation/settings" size="20px" />
  </div>
</div>

<style lang="scss">

.navbar {
  width: 100%;
  height: 48px;
  min-height: 48px;

  display: flex;
  justify-content: flex-end;
  align-items: center;

  background-color: #212327;
  border-top: 1.5px solid #ffffff10;

  .item {
    width: 36px;
    height: 36px;

    margin-right: 6px;

    display: flex;
    justify-content: center;
    align-items: center;

    color: #888;
    border-radius: 8px;

    &:hover {
      background-color: #ffffff0b;
      color: #ccc;
    }
  }
}

</style>