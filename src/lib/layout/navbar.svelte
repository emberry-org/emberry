<script lang="ts">
  import { afterNavigate, goto } from "$app/navigation";
  import Icon from "@iconify/svelte";

  let path = "";

  afterNavigate((nav) => {
    if (nav.to) path = nav.to.route.id ?? "";
  });
</script>


<nav>
  <button class:is-active={ path === "" } on:click={() => goto('/')}>
    <Icon icon="bry:explore" width="24px" />
  </button>

  <button class="move-bottom">
    <Icon icon="bry:settings" width="24px" />
  </button>
</nav>


<style lang="scss">

nav {
  width: 72px;
  min-width: 72px;

  display: flex;
  flex-direction: column;
  align-items: center;

  padding: 12px 0 0 0;

  button {
    position: relative;
    width: 48px;
    height: 48px;
    margin-bottom: 12px;

    color: #aaa;

    border-radius: 12px;
    background-color: #ffffff08;
    border: none;
    transition: color 0.1s;

    &:not(&.is-active):hover {
      color: #ED9450;
      cursor: pointer;
    }

    &:not(&.is-active):active {
      background-color: #ffffff18;
      cursor: pointer;
    }

    &.is-active {
      color: #ED9450;
    }

    &.is-active:after {
      content: "";
      position: absolute;

      top: 50%;
      transform: translateY(-50%);
      left: -13px;
      width: 5px;
      height: 24px;

      background-color: #ED9450;
      border-radius: 0 4px 4px 0;

      pointer-events: none;
    }
  }

  .move-bottom {
    margin-top: auto;
  }
}

</style>