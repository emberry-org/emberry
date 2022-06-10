<script lang="ts">
  import { useNavigate } from "svelte-navigator";

  import Icon from "../comps/Icon.svelte";

  const nav = useNavigate();
  nav('chat/abc123');
</script>

<div class="home">
  <div class="sidebar">

  </div>

  <div class="content">
    <div class="titlebar">
      <div class="block">
        
      </div>
      <button class="icon-button">
        <Icon name="terminal" size="20px" />
      </button>
      <button class="icon-button" disabled>
        <Icon name="terminal" size="20px" />
      </button>
      <button class="icon-button">
        <Icon name="" size="20px" />
      </button>

      <button class="icon-button" style="margin-left: auto;">
        <Icon name="call" size="20px" />
      </button>
      <button class="icon-button">
        <Icon name="addressBook" size="20px" />
      </button>
    </div>
    <div style="display: flex;">
      <p class="text-header" style="margin-left: 16px; margin-right: 12px;">Header</p> <p class="text-small">Some text</p>
    </div>
    <p class="text-normal" style="margin-left: 16px;">Some text</p>
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

      background-color: #242424;
      border-right: 1.5px solid #383838;
    }

    .content {
      flex-grow: 1;
      height: 100%;

      .titlebar {
        width: calc(100% - 8px);
        height: calc(46px - 16px);

        display: flex;

        padding: 8px;
        padding-bottom: 8px;

        border-bottom: 1.5px solid #ffffff11;

        .block {
          background-image: linear-gradient(-45deg, #22ce53,#52ec7d);
          border-radius: 4px;
          margin-left: 4px;
          margin-right: 20px;
        }
      }
    }
  }
</style>
