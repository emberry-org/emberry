<script lang="ts">
  import { Content, Explorer, Pub } from "@page/structure";
  import NavBar from "./Navbar.svelte";

</script>

<div class="body">
  
  <!-- <Pub /> -->

  <NavBar />

  <Explorer />

  <Content />

  <!-- <Explorer /> -->

</div>

<style lang="scss">

.body {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: row;
}

</style>