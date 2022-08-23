<script lang="ts">
  import Content from "@router/Content.svelte";
  import { Navbar } from "@win";

</script>

<div class="body">

  <Navbar />
  
  <Content />

</div>

<style lang="scss">

.body {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: row;
}

</style>