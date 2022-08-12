<script lang="ts">
  
</script>

<div class="side-panel">
  <div class="toolbar">

  </div>
  <div class="panel">

  </div>
</div>

<style lang="scss">

.side-panel {
  width: 248px; //calc(248px - 3px);
  min-width: 248px; //calc(248px - 3px);
  flex-grow: 1;

  display: flex;
  flex-direction: column;
  //border-left: 1.5px solid #fff2;
  //border-right: 1.5px solid #fff2;

  .toolbar {
    width: 100%;
    height: 30.5px;

    background-color: var(--mg);
  }

  .panel {
    width: 100%;
    flex-grow: 1;

    background-color: var(--mg);
  }
}

</style>