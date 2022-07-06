<script lang="ts">
  export let arrow: 'true' | 'false' = 'true';
  export let orientation: 'se' | 'sw' = 'sw';
  export let margins: string = '0px 0px 0px 0px';

  let modal: HTMLDivElement;
  let isopen: boolean = false;

  $: isopen, onToggle();

  const onToggle = () => {
    if (isopen == true) {
      /* Check if the user clicks outside the panel */
      document.addEventListener("mousedown", onMouseDown, true);
      setTimeout(() => {
        if (modal && modal.querySelector("#close-btn")) modal.querySelector("#close-btn").addEventListener("click", onCloseButton, true);
      }, 100);
    } else {
      /* Remove the event listener after the element is destroyed */
      document.removeEventListener("mousedown", onMouseDown, true);
      if (modal && modal.querySelector("#close-btn")) modal.querySelector("#close-btn").removeEventListener("click", onCloseButton, true);
    }
  };

  /** Called when the user clicks somewhere in the app */
  const onMouseDown = (e: MouseEvent) => {
    if (isopen == false || !modal) return;

    const bounds: DOMRect = modal.getBoundingClientRect();

    /* Close the command center if the users clicks outside of its bounds */
    if (e.x <= bounds.left || e.x >= bounds.right || e.y <= bounds.top || e.y >= bounds.bottom) {
      isopen = false;
    }
  };

  /** Called when the close button is clicked */
  const onCloseButton = () => {
    isopen = false;
  };

</script>

<div class="modal { orientation }">
  
  <div class="btn" on:mousedown={() => isopen = true}>
    <slot name="btn" >

    </slot>
  </div>

  {#if isopen}

  <div class="popup { arrow == 'true' ? 'arrow' : '' }" style="margin: { margins }" bind:this={modal}>
    <slot name="mdl" >
      <button id="close-btn" />
    </slot>
  </div>

  {/if}
</div>

<style lang="scss" global>
  @import './modal.scss';
</style>