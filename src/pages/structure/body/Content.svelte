<script lang="ts">
  import { Home, Chat } from "@page/content";
  import { Router, Route } from "svelte-navigator";
import Statusbar from "./Statusbar.svelte";

</script>

<div class="content">
  <Router primary={false}>
    <Route path="/">
      <div class="wrapper">
        <Home />
      </div>
    </Route>
    <Route path="/chat/:id" let:params>
      <div class="wrapper">
        <Chat id={params.id} />
      </div>
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
  position: relative;
  margin-left: 16px;
  //border-left: 1.5px solid var(--fg);
  //border-bottom-left-radius: 12px;
  //border-bottom: 1.5px solid #37393d;

  .wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }
}

</style>