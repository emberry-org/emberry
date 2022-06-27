<script lang="ts">
  import { oppSys } from "@store";

  $: noBlur = $oppSys != 'windows' && $oppSys != 'darwin';

</script>

<div class="side-panel { noBlur ? 'solid' : '' }">

</div>

<style lang="scss">

.side-panel {
  width: 200px;
  min-width: 200px;
  height: calc(100% - 2px);

  &.solid {
    background-color: #1a2320;
  }
}

</style>