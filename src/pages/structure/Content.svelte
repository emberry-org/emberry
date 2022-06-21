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
  height: calc(100% - 2px);

  background-color: #333333;
  
  border-top: 2px solid #343434;
  border-left: 2px solid #343434;
  border-top-left-radius: 12px;
}

</style>