<script lang="ts">
  import Icon from "@lib/Icon.svelte";

  let modal: HTMLDivElement;
  let isopen: boolean = false;

  $: isopen, onToggle();

  const onToggle = () => {
    if (isopen == true) {
      /* Check if the user clicks outside the panel */
      document.addEventListener("mousedown", onMouseDown, true);
    } else {
      /* Remove the event listener after the element is destroyed */
      document.removeEventListener("mousedown", onMouseDown, true);
    }
  };

  /** Called when the user clicks somewhere in the app */
  const onMouseDown = (e: MouseEvent) => {
    if (isopen == false) return;

    const bounds: DOMRect = modal.getBoundingClientRect();

    /* Close the command center if the users clicks outside of its bounds */
    if (e.x <= bounds.left || e.x >= bounds.right || e.y <= bounds.top || e.y >= bounds.bottom) {
      isopen = false;
    }
  };

</script>

<div class="room-request-modal">
  <div class="invis-btn" on:mousedown={() => isopen = true} />

  {#if isopen}
    <div class="modal" bind:this={modal}>
      <div class="header">
        <h3> Username </h3>
        <p> #1234 </p>
        <button class="close-btn" on:click={() => isopen = false}>
          <Icon name="window/close" size="20px" />
        </button>
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .room-request-modal {
    position: absolute;
    top: 0;
    left: 0;

    width: 34px;
    height: 34px;

    .invis-btn {
      width: 46px;
      height: 44px;

      position: absolute;
      top: -4px;
      left: -6px;

      cursor: pointer;
    }

    .modal {
      position: absolute;

      width: 260px;
      height: 128px;

      top: 44px;
      right: -8px;

      margin-right: 5px;

      background-color: #37383c;
      border: 1.5px solid #535557;
      border-radius: 6px;
      box-shadow: 0 8px 24px #1c1c1c;

      animation: modal-animation 0.2s cubic-bezier(0, 0.1, 0.1, 1) backwards;

      @keyframes modal-animation {
        0% {
          opacity: 0;
          top: 28px;
        }
        100% {
          opacity: 1;
          top: 44px;
        }
      }

      &::before {
        content: "";
        position: absolute;

        top: -16px;
        right: 12.5px;
        left: auto;

        border: 8px solid #0000;
        border-bottom-color: #535557;
      }

      &::after {
        content: "";
        position: absolute;

        top: -13.5px;
        right: 13px;
        left: auto;

        border: 7.5px solid #0000;
        border-bottom-color: #37383c;
      }
      
      .header {
        width: 100%;
        height: 32px;

        display: flex;
        align-items: center;

        border-bottom: 1.5px solid #434547;

        h3 {
          font-size: 14px;
          font-weight: normal;
          margin-left: 16px;
        }

        p {
          font-size: 12px;
          font-weight: normal;
          margin-left: 8px;
          padding-top: 2px;
        }

        .close-btn {
          width: 32px;
          height: 32px;

          display: flex;
          justify-content: center;
          align-items: center;

          margin-left: auto;
          margin-right: 4px;

          background-color: #0000;
          border: none;
          outline: none;
          color: #888;

          cursor: pointer;

          &:hover {
            color: #ddd;
          }
        }
      }
    }
  }
</style>