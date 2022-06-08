<script lang="ts">
  import Icon from "../comps/Icon.svelte";

</script>

<div class="home">
  <div class="sidebar">

  </div>

  <div class="content">
    <div class="titlebar">
      <button>
        <Icon name="terminal" size="20px" />
      </button>
      <button>
        <Icon name="terminal" size="20px" />
      </button>
      <button>
        <Icon name="terminal" size="20px" />
      </button>

      <button style="margin-left: auto;">
        <Icon name="terminal" size="20px" />
      </button>
      <button>
        <Icon name="terminal" size="20px" />
      </button>
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

      background-color: #262626;
      border-right: 1.5px solid #323232;
    }

    .content {
      flex-grow: 1;
      height: 100%;

      .titlebar {
        width: calc(100% - 8px);
        height: calc(46px - 16px);

        display: flex;

        padding: 8px;

        border-bottom: 1.5px solid #323232;

        button {
          width: 30px;
          height: 30px;

          margin-right: 8px;
          padding: 0;

          display: flex;
          justify-content: center;
          align-items: center;

          background-color: transparent;
          color: #999999;
          border-radius: 6px;
          border: none;
          outline: none;
          cursor: pointer;

          &:disabled {
            background-color: transparent !important;
            color: #4c4c4c;
            cursor: default;
          }

          &:hover {
            background-color: #ffffff11;
            color: #cccccc;
          }
        }
      }
    }
  }
</style>
