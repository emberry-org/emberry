<script lang="ts">
  import { Content, SidePanel, Navbar } from "@page/structure";

</script>

<div class="body">

  <Navbar />
  
  <div class="left-side">
    <SidePanel />
    
    <!-- <Navbar /> -->
  </div>

  <Content />

</div>

<style lang="scss">

.body {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: row;

  .left-side {
    width: 248px;
    height: 100%;

    display: flex;
    flex-direction: column;

    //border-bottom: 1.5px solid #ffffff10;
  }
}

</style>