<script lang="ts">
  import Icon from "@lib/Icon.svelte";
  import { navigate } from "svelte-navigator";
</script>

<div class="home">
  <div class="card">
    <Icon name="logo" size="50vh" />
    <div class="buttons">
      <button class="card-button start" on:click={() => navigate('/chat/new', { replace: true })}>New Chat</button>
    </div>
  </div>
</div>

<style lang="scss">
  .home {
    width: 100%;
    height: 100%;

    display: flex;
    justify-content: center;
    align-items: center;

    color: #212327;

    .card {
      height: fit-content;
      display: flex;
      flex-direction: column;

      .buttons {
        width: 100%;
        height: 36px;

        margin-top: 24px;

        display: flex;
        justify-content: space-around;

        .card-button {
          width: 30%;
          height: 36px;

          background-color: #37383c;
          border: 1.5px solid #ffffff18;
          box-shadow: 0 1px 2px 0 #00000055;
          border-radius: 6px;

          padding: 0 12px 0 12px;

          font-family: Inter;
          font-size: 0.85em;
          color: #bbb;

          user-select: none;
          cursor: pointer;

          &.start {
            width: 40%;
            min-width: fit-content;
          }

          &:hover {
            border: 1.5px solid #ffffff33;
          }
        }
      }

      :global(svg) {
        max-width: 200px;
        max-height: 200px;
      }
    }
  }
</style>
