<script lang="ts">
  import { Home, Chat } from "@page/content";
  import { Router, Route } from "svelte-navigator";
  import Footer from "./Footer.svelte";

</script>

<div class="content">

  <div class="router">
    <Router primary={false}>
      <Route path="/">
        <Home />
      </Route>
      <Route path="/chat/:id" let:params>
        <Chat id={params.id} />
      </Route>
    </Router>
  </div>
  
  <Footer />

</div>

<style lang="scss">

.content {
  flex-grow: 1;

  display: flex;
  flex-direction: column;

  .router {
    width: 100%;
    max-height: calc(100vh - 76px);
    flex-grow: 1;

    background-color: #292b2f;
    border-left: 1.5px solid #ffffff16;
  }
}

</style>