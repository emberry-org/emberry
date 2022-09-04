<!-- A generic side panel component that is resizeable -->

<script lang="ts">
  import { onMount } from "svelte";


  export let position: "left" | "right" = "left";

  let panel: HTMLDivElement;
  let handle: HTMLDivElement;

  let resizing: boolean = false;

  function onResize(e: MouseEvent) {
    window.addEventListener('mousemove', mousemove);
    window.addEventListener('mouseup', mouseup);
    
    let prevX = e.x;
    const bounds = panel.getBoundingClientRect();
    resizing = true;
    
    // While moving the mouse update the width of the panel.
    function mousemove(e: MouseEvent) {
      e.preventDefault();
      if (position == "left") {
        let newX = prevX - e.x;
        panel.style.width = bounds.width - newX + "px";
      } else {
        let newX = prevX - e.x;
        panel.style.width = bounds.width + newX + "px";
      }
    }
    
    // If mouse up then remove event listeners.
    function mouseup() {
      window.removeEventListener('mousemove', mousemove);
      window.removeEventListener('mouseup', mouseup);
      resizing = false;
    }
  }

  onMount(() => {
    handle.addEventListener('mousedown', onResize);
  });

</script>


<div class="side-panel { position }" bind:this={ panel }>
  <slot />
  <div class="handle{ resizing ? " resizing" : "" }" bind:this={ handle } />
</div>


<style lang="scss">

.side-panel {
  position: relative;
  min-width: 240px;
  width: 240px;
  height: 100%;

  &.left {
    margin-right: 24px;

    .handle {
      position: absolute;
      top: 0;
      right: -18px;

      width: 12px;
      height: 100%;

      cursor: ew-resize;
    }
  }

  &.right {
    margin-left: 24px;

    .handle {
      position: absolute;
      top: 0;
      left: -18px;
      
      width: 12px;
      height: 100%;

      cursor: ew-resize;
    }
  }

  .handle {
    opacity: 0;
    transition: opacity 0.2s ease-out 0s;
    -webkit-transition: opacity 0.2s ease-out 0s;

    &:hover {
      opacity: 1;
      transition: opacity 0.2s ease-out 0.4s;
      -webkit-transition: opacity 0.2s ease-out 0.4s;
    }

    &::before {
      content: " ";
      position: absolute;
      
      width: 4px;
      height: 100%;
      margin-left: 4px;

      background-color: #6BA0D666;
      
      border-bottom-left-radius: 3px;
      border-bottom-right-radius: 3px;
    }

    &.resizing {
      opacity: 1;
    }
  }
}

</style>