<script lang="ts">
	export let sender: string;
	export let content: string;
</script>

<div class='msg'>
	<h3>{sender}</h3>
  <div class="body">
    <p>{content}</p>
  </div>
</div>

<style lang="scss">
  .msg {
    width: 100%;
    height: 64px;

    margin-bottom: 12px;

    h3 {
      height: 24px;
      color: #fff;
      font-weight: normal;
      font-size: 16px;
      margin: 0;
    }

    .body {
      background-color: #0002;
      height: fit-content;
      width: fit-content;
      padding: 8px 16px 6px 16px;
      border-radius: 10px;

      p {
        height: 24px;
        color: #aaa;
        font-size: 14px;
      }
    }
  }
</style>
