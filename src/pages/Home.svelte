<script>
</script>

<div class="home">
  <div class="chat">
    <div class="log">

    </div>
    <div class="input">
      <input type="text" placeholder="Message your peers">
    </div>
  </div>

  <div class="network">
    <div class="user" />
    <button class="button">disabled</button>
    <button class="button succes">join</button>
    <button class="button warning">leave</button>
    <button class="button link">info</button>
  </div>
</div>

<style lang="scss">
  .home {
    display: flex;
    width: 100%;
    height: 100%;

    .chat {
      width: 200px;
      height: calc(100% - 2px);
      display: flex;
      flex-direction: column;

      background-color: var(--foreground);
      border-right: 2px solid var(--foreground-border);
      border-top: 2px solid var(--foreground-border);

      .log {
        flex-grow: 1;
        width: 100%;
      }

      .input {
        width: 100%;
        height: 48px;
        display: flex;
        justify-content: center;
        align-items: center;

        input {
          width: calc(100% - 40px);
          height: calc(100% - 20px);
          padding: 0 10px 0 10px;

          color: var(--text);
          background-color: var(--foreground);
          border: 2px solid var(--foreground-border);
          border-radius: 8px;
          outline: none;

          &:focus {
            border-color: var(--lowlight);
          }

          &::placeholder {
            color: var(--lowlight);
          }
        }
      }
    }

    .network {
      flex-grow: 1;
      flex-wrap: wrap;
      display: flex;
      max-height: calc(100% - 2px);
      height: fit-content;

      background-color: var(--middleground);
      border-top: 2px solid var(--middleground-border);
      overflow: hidden;
      font-family: Josefin;

      .user {
        margin-left: 20px;
        margin-top: 20px;

        height: 60px;
        width: 60px;

        background-color: var(--foreground);
        border: 2px solid var(--foreground-border);
        border-radius: 10px;
      }
    }
  }
</style>
