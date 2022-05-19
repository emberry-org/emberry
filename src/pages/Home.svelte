<script lang="ts">
</script>

<div class="home">
  <div class="network">
    <div class="user" />
    <div class="user" />
    <div class="user" />
    <div class="user" />
  </div>

  <div class="chat">
    <div class="log">
      {#if true}
        <div class="info-card">
          <div class="logo" />

          <div class="info">
            <div class="lines">
              <div class="line">Create room</div>
              <div class="line">Join room</div>
              <div class="line">Exit room</div>
            </div>

            <div class="actions">
              <div class="action">
                <span class="action">Ctrl</span> + <span class="action">R</span>
              </div>
              <div class="action">
                <span class="action">Ctrl</span> + <span class="action">F</span>
              </div>
              <div class="action">
                <span class="action">Ctrl</span> + <span class="action">Q</span>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
    <div class="input">
      <input type="text" placeholder="Message your peers" />
    </div>
  </div>
</div>

<style lang="scss">
  .home {
    display: flex;
    width: 100%;
    height: 100vh; // calc(100vh - 44px);
    overflow: hidden;

    .chat {
      flex-grow: 1;
      height: calc(100% - 2px);
      display: flex;
      flex-direction: column;

      background-color: var(--foreground);
      border-top: 2px solid var(--foreground-border);

      .log {
        flex-grow: 1;
        width: 100%;

        .info-card {
          width: 100%;
          height: 100%;

          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;

          .logo {
            width: 200px;
            height: 200px;

            background-color: var(--middleground);
            border-radius: 25px;
          }

          .info {
            width: 100%;
            height: fit-content;
            margin-top: 42px;

            display: flex;
            justify-content: center;
            align-items: flex-start;

            .lines {
              width: 50%;
              height: fit-content;
              padding-right: 10px;

              .line {
                width: 100%;
                height: 31px;

                text-align: end;
                color: #6e7371;
              }
            }

            .actions {
              width: 50%;
              height: fit-content;
              padding-left: 10px;

              > .action {
                text-align: start;
                color: #828785;
                margin-top: -1px;
                height: 32px;

                span.action {
                  background-color: var(--foreground-border);
                  padding: 4px 6px 1px 6px;
                  border-radius: 4px;
                  font-size: 0.85rem;
                }
              }
            }
          }
        }
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

          font-family: Josefin;
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
      width: 50px;
      min-width: 50px;
      height: calc(100% - 2px);
      display: flex;
      flex-direction: column;

      background-color: var(--middleground);
      border-top: 2px solid var(--foreground-border);
      overflow: scroll;
      overflow-x: hidden;
      font-family: Josefin;

      &::-webkit-scrollbar {
        width: 0;
        background: transparent;
      }

      .user {
        margin-left: 7px;
        margin-top: 7px;

        height: 32px;
        min-height: 32px;
        width: 32px;

        background-color: var(--foreground);
        border: 2px solid var(--foreground-border);
        border-radius: 6px;
      }
    }
  }
</style>
