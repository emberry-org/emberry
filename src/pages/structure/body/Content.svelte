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

  display: flex;
  flex-direction: column;

  background-color: #292a2c;
  border-bottom: 1.5px solid #37393d;
}

</style>