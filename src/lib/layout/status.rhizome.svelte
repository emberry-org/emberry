<script lang="ts">
  import Icon from "@iconify/svelte";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  /** The current status of the rhizome connection */
  let rzStatus: string = "connecting";
  /** Time since connect was called */
  let rzMili: string = "0";

  onMount(() => {
    // Try to load the rhizome status from the session storage.
    rzStatus = sessionStorage.getItem("rz-status") ?? "connecting";
    rzMili = sessionStorage.getItem("rz-mili") ?? "0";

    listen("rz-con", (evt) => setStatus("connected", evt.payload as string));
    listen("rz-dc", (evt) => setStatus("disconnected", evt.payload as string));
    listen("rz-f", (evt) => setStatus("failed", evt.payload as string));
  });

  function setStatus(status: string, mili: string) { 
    rzStatus = status; 
    sessionStorage.setItem("rz-status", status); 

    if (mili !== "null" && mili !== null) {
      rzMili = mili;
      sessionStorage.setItem("rz-mili", mili);
    }
  }
</script>


<button class="rhizome { rzStatus } move-right" title="{ rzMili }ms">
  {#if rzStatus === "connecting" || rzStatus === "failed"}
  <Icon icon="bry:rz-disc" width="16px" height="16px" />
  {:else}
  <Icon icon="bry:rz-conn" width="16px" height="20px" />
  {/if}
</button>


<style lang="scss">

button {
  position: relative;
  max-height: 22px;
  margin: 0;
  padding: 0 3px;

  border: none;
  outline: none;
  cursor: pointer;
  border-top-left-radius: 8px;

  display: flex;
  justify-content: center;
  align-items: center;

  background-color: #fff0;
  color: #aaa;
  transition: color 0.1s;
}

.disconnected { color: #444; }
.failed { color: rgb(217, 82, 82); }
.connected { color: #888; &:hover { color: #f9c15c; } }
.connecting { color: #aaa; }

</style>

