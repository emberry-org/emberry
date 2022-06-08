<script lang="ts">
  import Icon from "../comps/Icon.svelte";

</script>

<div class="home">
  <div class="sidebar">

  </div>

  <div class="content">
    <div class="titlebar">

    </div>
  </div>
</div>

<style lang="scss">
  .home {
    display: flex;
    width: 100%;
    height: 100vh; // calc(100vh - 44px);
    overflow: hidden;

    .sidebar {
      width: 200px;
      height: 100%;

      background-color: #2b2b2b;
      border-right: 2px solid #333333;
    }

    .content {
      flex-grow: 1;
      height: 100%;

      .titlebar {
        width: 100%;
        height: 28px;

        border-bottom: 2px solid #2b2b2b;
      }
    }
  }
</style>
