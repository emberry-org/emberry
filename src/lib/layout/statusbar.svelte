<script lang="ts">
  import RhizomeStatus from "./status.rhizome.svelte";
</script>


<footer>
  <RhizomeStatus />
</footer>


<style lang="scss">

footer {
  width: calc(100% + 20px);
  height: 22px;
  margin: 20px 0 0 -20px;

  background-color: var(--bg-100);
  display: flex;

  :global(.move-right) {
    margin-left: auto;
  }
}

</style>

