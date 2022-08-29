<script lang="ts">
	export let sender: string;
	export let content: string;
</script>

<div class='msg'>
	<h3>{sender}</h3>
	<p>{content}</p>
</div>

<style lang="scss">
  .msg {
    width: 100%;
    height: 48px;

    h3 {
      height: 24px;
      color: #fff;
      margin: 0;
    }

    p {
      height: 24px;
      color: #aaa;
    }
  }
</style>
