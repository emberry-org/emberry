<script lang="ts">
  import { afterNavigate, goto } from "$app/navigation";
  import Icon from "@iconify/svelte";

  let path = "/";

  afterNavigate((nav) => {
    if (nav.to) path = nav.to.route.id ?? "/";
  });
</script>


<nav>
  <div class="logo">
    <Icon icon="bry:logo" width="28px" />
  </div>

  <button class:is-active={ path === "/" } on:click={() => goto('/')}>
    <Icon icon="16x16:telescope" width="24px" />
  </button>

  <button class="move-bottom">
    <Icon icon="bry:settings" width="24px" />
  </button>

  <button class:is-active={ path.startsWith('/chat/') } on:click={() => goto('/chat/debug')}>
    <Icon icon="cmd:debug" width="24px" />
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
  margin-right: 20px;
  background-color: var(--bg-100);

  button {
    position: relative;
    width: 48px;
    height: 48px;
    margin-bottom: 12px;

    border-radius: 12px;
    color: #6e6660;
    border: none;
    transition: color 10ms;
    background-color: transparent;
    cursor: pointer;

    &:not(&.is-active):hover {
      color: #cfbaa7;
      cursor: pointer;
    }

    &:hover {
      background-color: var(--bg-100-active);
    }

    &.is-active {
      color: #cfbaa7;
    }

    &.is-active:after {
      content: "";
      position: absolute;

      top: 50%;
      transform: translateY(-50%);
      left: -13px;
      width: 5px;
      height: 16px;

      background-color: #cfbaa7;
      border-radius: 0 4px 4px 0;

      pointer-events: none;
    }
  }

  .logo {
    position: relative;
    width: 36px;
    height: 46px;

    margin-bottom: 12px;
    padding: 1px 6px;

    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--primary);
  }

  .move-bottom {
    margin-top: auto;
  }
}

</style>
