<script lang="ts">
  import { parseContent } from "./msg.parser";

  // Standard message variables.
	export let sender: string;
	export let content: string;
  export let time: string;
  export let chain: boolean;
</script>


<div class='msg'>
	{#if !chain}
    <div class="profile-picture" />
    <h3>{ sender } <span>{ time }</span></h3>
  {/if}
  <div class="body">
    <p>{ @html parseContent(content) }</p>
  </div>
</div>


<style lang="scss">
  // Normal message
  .msg {
    position: relative;
    width: 100%;
    height: fit-content;

    margin-bottom: 6px;
    padding-left: 42px;
    font-weight: 400;

    .profile-picture {
      position: absolute;

      left: 0;
      top: -3px;
      width: 34px;
      height: 34px;

      background-color: #1c1c1c;
      background-size: contain;
      border-radius: 8px;

      user-select: none;
      -webkit-user-select: none;

      // Picture Translucent Outline
      &::after {
        content: "";

        width: 30px;
        height: 30px;

        position: absolute;
        top: 0;
        left: 0;

        border: 2px solid #ffffff10;
        border-radius: 8px;
      }
    }

    h3 {
      height: 24px;
      color: #ddd;
      font-weight: 400;
      font-size: 14px;
      margin: 16px 0 0 0;

      span {
        font-size: 10px;
        color: #777;
        font-weight: 500;
        margin-left: 10px;
      }
    }

    .body {
      background-color: #282828;
      height: fit-content;
      width: fit-content;
      padding: 0px 12px 0px 12px;
      margin: 4px 0 0 0;
      border-radius: 10px;

      p {
        height: fit-content;
        color: #ddd;
        font-size: 14px;
        white-space: pre-wrap;
        line-height: 14px;
        margin: 10px 0 12px 0;

        :global(a) {
          font-weight: inherit;
          color: #6BA0D6;

          &:hover {
            text-decoration: underline;
          }
        }
      }
    }
  }
</style>
