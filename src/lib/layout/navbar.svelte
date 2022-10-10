<script lang="ts">
  import { goto } from "$app/navigation";
  import Icon from "@iconify/svelte";
</script>


<nav>
  <button on:click={() => goto('/')}>
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

  padding: 12px 0;

  button {
    width: 48px;
    height: 48px;

    color: #aaa;

    border-radius: 12px;
    background-color: #ffffff08;
    border: none;

    cursor: pointer;

    &:hover {
      background-color: #ffffff18;
      color: #eee;
    }
  }

  .move-bottom {
    margin-top: auto;
  }
}

</style>