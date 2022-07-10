<script lang="ts">
  let dialog: HTMLDivElement;
  let btn: HTMLDivElement;
  let isopen: boolean = false;

  $: isopen, onToggle();

  const onToggle = () => {
    if (isopen == true) {
      /* Check if the user clicks outside the panel */
      document.addEventListener("mousedown", onMouseDown, true);
      setTimeout(() => {
        if (dialog && dialog.querySelector("#close-btn")) dialog.querySelector("#close-btn").addEventListener("click", onCloseButton, true);
      }, 100);
    } else {
      /* Remove the event listener after the element is destroyed */
      document.removeEventListener("mousedown", onMouseDown, true);
      if (dialog && dialog.querySelector("#close-btn")) dialog.querySelector("#close-btn").removeEventListener("click", onCloseButton, true);
    }
  };

  /** Called when the user clicks somewhere in the app */
  const onMouseDown = (e: MouseEvent) => {
    if (isopen == false || !dialog) return;

    const modalBounds: DOMRect = dialog.getBoundingClientRect();
    const btnBounds: DOMRect = btn.getBoundingClientRect();

    /* Close the dialog if the users clicks outside of its bounds */
    if ((e.x <= modalBounds.left || e.x >= modalBounds.right || e.y <= modalBounds.top || e.y >= modalBounds.bottom) &&
        (e.x <= btnBounds.left || e.x >= btnBounds.right || e.y <= btnBounds.top || e.y >= btnBounds.bottom)) {
      isopen = false;
    }
  };

  /** Called when the close button is clicked */
  const onCloseButton = () => {
    isopen = false;
  };

</script>

<div class="dialog">
  
  <div class="btn" on:mousedown={() => isopen = !isopen} bind:this={btn}>
    <slot name="btn" >

    </slot>
  </div>

  {#if isopen}

  <div class="overlay" />

  <div class="popup" bind:this={dialog}>
    <slot name="mdl" >
      <button id="close-btn" />
    </slot>
  </div>

  {/if}
</div>

<style lang="scss" global>
  @import './dialog.scss';
</style>