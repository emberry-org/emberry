<script lang="ts">

  export let id: string;

</script>

<div class="chat">

  <div class="toolbar">

  </div>

  <div class="logs">

  </div>

  <div class="input">
    <input type="text">
  </div>

</div>

<style lang="scss">

.chat {
  width: 100%;
  height: 100%;

  display: flex;
  flex-direction: column;

  .toolbar {
    width: 100%;
    height: 24px;

    background-color: #37383c;
    border-bottom: 1.5px solid #434547;
    box-shadow: 0 1px 2px 0 #00000055;
  }

  .logs {
    width: 100%;
    flex-grow: 1;
  }

  .input {
    width: 100%;
    height: 60px;

    display: flex;
    justify-content: center;
    align-items: center;
    
    input {
      width: calc(100% - 32px);
      height: 28px; 

      box-shadow: 0 1px 2px 0 #00000055;

      background-color: #37383c;
      border: 1.5px solid #434547;
      outline: none;
      border-radius: 4px;
    }
  }
}

</style>