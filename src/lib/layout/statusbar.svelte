<script lang="ts">
  import Icon from "@iconify/svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let rzStatus: string = "connecting";

  onMount(() => {
    // Try to load the rhizome status from the session storage.
    rzStatus = sessionStorage.getItem("rz-status") ?? "connecting";

    listen("rz-con", () => setStatus("connected"));
    listen("rz-dc", () => setStatus("disconnected"));
    listen("rz-f", () => setStatus("failed"));
  });

  function setStatus(status: string) { rzStatus = status; sessionStorage.setItem("rz-status", status); }
</script>


<footer>
  <button class="rhizome { rzStatus } move-right">
    {#if rzStatus === "connecting"}
    <Icon icon="bry:loading" width="16px" height="16px" />
    {:else}
    <Icon icon="bry:rhizome-status" width="16px" height="16px" />
    {/if}
  </button>
</footer>


<style lang="scss">

footer {
  width: 100%;
  height: 22px;

  display: flex;

  .move-right {
    margin-left: auto;
  }

  button {
    max-height: 22px;
    margin: 0;
    padding: 0 3px;

    border: none;
    outline: none;

    display: flex;
    justify-content: center;
    align-items: center;

    background-color: #fff0;
    color: #aaa;
    
    &.disconnected { color: #555; }
    &.failed { color: rgb(217, 82, 82); }
    &.connected { color: #aaa; }
    &.connecting { color: #ccc; }
  }
}

</style>