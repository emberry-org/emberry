<script lang="ts">
  import { Home, Chat } from "@pages";
  import { Router, Route } from "svelte-navigator";

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
</div>

<style lang="scss">

.content {
  flex-grow: 1;

  display: flex;
  flex-direction: column;

  background-color: var(--bg);
  position: relative;

  .wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }
}

</style>