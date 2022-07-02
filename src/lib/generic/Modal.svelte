<script lang="ts">
  export let details: HTMLDetailsElement = undefined;

</script>

<details
  bind:this={details}
  class="details"
>
  <summary class="summary">
    <slot name="summary">
      
    </slot>
  </summary>

  <div class="menu" role="menu">
    <slot name="details-menu">

    </slot>
  </div>
</details>

<style lang="scss" global>
  .details {
    position: relative !important;

    &[open] > .summary::before {
      position: fixed;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      z-index: 80;
      display: block;
      cursor: default;
      content: " ";
      background: transparent;
    }

    &:not([open]) > :not(summary) {
      display: none !important;
    }

    // Summary class
    .summary {
      width: 24px;
      height: 24px;

      background-color: #0000;
      color: #fff;

      user-select: none;
      -webkit-user-select: none;
      list-style: none;

      &::marker {
        content: "";
      }
    }

    // Menu class
    .menu {
      width: fit-content;
      height: fit-content;

      background-color: #0000;
      color: #fff;

      position: absolute;
      top: 5px;
      right: 0;

      z-index: 99;
      display: flex;
      padding: 12px 0 12px 12px;
      flex-direction: column;
      list-style: none;

      animation: modal-animation 0.12s cubic-bezier(0, 0.1, 0.1, 1) backwards;

      @keyframes modal-animation {
        0% {
          opacity: 0;
          transform: translateY(-16px);
        }
      }
    }
  }
</style>