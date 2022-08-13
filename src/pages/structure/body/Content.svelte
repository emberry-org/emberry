<script lang="ts">
  import { Home, Chat } from "@page/content";
  import { Router, Route } from "svelte-navigator";
import Statusbar from "./Statusbar.svelte";

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

  <Statusbar />
</div>

<style lang="scss">

.content {
  flex-grow: 1;

  display: flex;
  flex-direction: column;

  background-color: var(--bg);
  //border-left: 1.5px solid var(--fg);
  //border-bottom-left-radius: 12px;
  //border-bottom: 1.5px solid #37393d;
}

</style>