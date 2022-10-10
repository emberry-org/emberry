<script lang="ts">
  import RhizomeStatus from "./status.rhizome.svelte";
</script>


<footer>
  <RhizomeStatus />
</footer>


<style lang="scss">

footer {
  width: 100%;
  height: 22px;

  display: flex;

  :global(.move-right) {
    margin-left: auto;
  }
}

</style>

