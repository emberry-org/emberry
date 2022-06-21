<script lang="ts">
  import Icon from "@lib/Icon.svelte";

</script>

<div class="pub">
  <div class="bar">
    <div class="peer">
      <div class="indicator">
        <Icon name="sprout" />
      </div>
    </div>
    <div class="peer">
      <div class="indicator">
        <Icon name="sprout" />
      </div>
    </div>
    <div class="peer">
      <div class="indicator">
        <Icon name="sprout" />
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  .pub {
    width: 66px;
    height: calc(100% - 1px);

    background-color: #222222;
    border-top: 1px solid #545454;

    .bar {
      width: 100%;
      height: 100%;

      display: flex;
      flex-direction: column;

      .peer {
        width: 36px;
        height: 36px;

        margin: 14px 14px 0 14px;

        background-color: #333333;
        border: 1px solid #545454;
        border-radius: 4px;

        position: relative;

        .indicator {
          position: absolute;

          width: 16px;
          height: 16px;

          top: -4px;
          right: -4px;

          background-color: #479b4a;
          border-radius: 4px;

          border: 2px solid #222;

          box-shadow: 0 0 0 2px #479b4a;

          :global(svg) {
            position: absolute;

            width: 12px;
            height: 12px;

            color: #fff;
            stroke-width: 1px;
            stroke: black;

            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
          }
        }
      }
    }
  }
</style>
