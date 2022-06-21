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
  height: calc(100% - 1px);

  background-color: #282828;
  
  border-top: 1px solid #545454;
  border-left: 1px solid #3f3f3f;
}

</style>