<!-- User.Me : a compact panel for user account information (displayed in the top right corner) -->

<script lang="ts">

</script>


<div class="row">
  <div class="info">
    <h3>Username</h3>
    <p>playing spelltanks</p>
  </div>
  <div class="avatar" />
</div>

<div class="row">
  <p class="desc">This is a place where you can add a description of your own :D</p>
</div>


<style lang="scss">

.row {
  height: 42px;
  display: flex;
  justify-content: flex-start;

  .avatar {
    width: 42px;
    height: 42px;

    background-color: #888;
    border-radius: 8px;
    margin-left: auto;
  }

  .info {
    display: flex;
    flex-direction: column;

    h3, p {
      margin: 0;
    }

    h3 {
      font-size: 16px;
      line-height: 21px;
    }

    p {
      font-size: 14px;
      line-height: 21px;
      color: #aaa;
    }
  }

  .desc {
    font-size: 14px;
    line-height: 21px;
    color: #aaa;
  }
}

</style>