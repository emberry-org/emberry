<script lang="ts">
  import { Home, Chat } from "@page/content";
  import { Router, Route } from "svelte-navigator";

</script>

<div class="content">

  <Router primary={false}>
    <Route path="/">
      <Home />
    </Route>
    <Route path="/chat/:id" let:params>
      <Chat id={params.id} />
    </Route>
  </Router>

</div>

<style lang="scss">

.content {
  flex-grow: 1;
  height: calc(100% - 1.5px);

  background-color: #343233;
  
  border-top: 1.5px solid #efdab928;
  border-right: 1.5px solid #efdab928;
  border-top-right-radius: 12px;
  z-index: 1;
  box-shadow: 0 2px 4px 0 #00000055;
}

</style>