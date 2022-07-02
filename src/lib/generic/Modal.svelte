<script lang="ts">
  let details: HTMLDetailsElement;
  let isopen = '';

  function onToggle() {
    isopen = details.open ? 'open' : '';
  }

</script>

<details
  bind:this={details}
  on:toggle={onToggle}
  data-view-component="true"
  class="details { isopen }"
>
  <summary class="summary">
    <slot name="summary">
      
    </slot>
  </summary>

  <details-menu class="menu" role="menu">
    <slot name="details-menu">

    </slot>
  </details-menu>
</details>

<style lang="scss" global>
  .details {
    position: relative !important;

    &.open .summary::before {
      position: fixed;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      z-index: 80;
      display: block;
      cursor: default;
      content: "";
      background: #0000;
    }

    // Placeholder summary class
    .summary {
      width: 24px;
      height: 24px;

      background-color: #0000;
      color: #fff;

      user-select: none;
      -webkit-user-select: none;

      &::marker {
        content: "";
      }
    }

    // Placeholder menu class
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
    }
  }
</style>