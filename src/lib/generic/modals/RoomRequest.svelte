<script lang="ts">
  import Modal from "@lib/generic/Modal.svelte";
  import Icon from "@lib/Icon.svelte";

  let details: HTMLDetailsElement;

</script>

<div class="room-request-modal">
  <Modal bind:details>
    <div class="invis-btn" slot="summary">
      
    </div>
  
    <div class="modal" slot="details-menu">
      <div class="header">
        <h3> Username </h3>
        <p> #1234 </p>
        <button class="close-btn" on:click={() => details.open = false}>
          <Icon name="window/close" size="20px" />
        </button>
      </div>
    </div>
  </Modal>
</div>

<style lang="scss">
  .room-request-modal {
    width: 44px;
    height: 44px;

    .invis-btn {
      width: 44px;
      height: 44px;

      position: absolute;
      top: -39px;
      left: 0;

      cursor: pointer;
    }

    .modal {
      width: 260px;
      height: 128px;

      margin-right: 5px;

      background-color: #37383c;
      border: 1.5px solid #535557;
      border-radius: 6px;
      box-shadow: 0 8px 24px #1c2128;

      &::before {
        content: "";
        position: absolute;

        top: -4px;
        right: 18.5px;
        left: auto;

        border: 8px solid #0000;
        border-bottom-color: #535557;
      }

      &::after {
        content: "";
        position: absolute;

        top: -2px;
        right: 19.25px;
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