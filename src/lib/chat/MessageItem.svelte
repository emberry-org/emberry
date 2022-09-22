<script lang="ts">
  import type { Msg } from "@core/messages/Msg";


  // Standard message variables.
	export let sender: string;
	export let content: string;
  export let time: string;
  export let chain: boolean;
  export let target: Msg | undefined = undefined;

  // Thread message variables.
  export let messages: Array<Msg> | undefined = undefined;
  let members: Array<String> = ['Member 1', 'Member 2', 'Member 3'];

  function applyTags(s: String) {
    return s.replace(/@(\S+)/gi,'<a>@$1</a>');
  }

</script>

{#if !messages} <!-- HTML for normal message -->

{#if target} <!-- HTML in case this is a reaction -->

<div class="reaction">
  <div class="profile-picture sm" />
  <h3>@{ target.sender }</h3>
  <p>{ @html applyTags(target.content) }</p>
</div>

{/if}

<div class='msg'>
	{#if !chain}
    <div class="profile-picture" />
    <h3>{ sender } <span>{ time }</span></h3>
  {/if}
  <div class="body">
    <p>{ @html applyTags(content) }</p>
  </div>
</div>

{:else} <!-- HTML for thread message -->

<div class="thread">
  <div class="profile-picture" />
  <div class="body">
    <p>{ @html applyTags(content) } <span>{ time }</span></p>
    <div class="footer">
      <div class="participants">
        {#each members as member}
        <div class="profile-picture sm" />
        {/each}
      </div>
      <span class="messages">•<span class="spacer" />{ messages.length } Messages</span>
    </div>
  </div>
</div>

{/if}

<style lang="scss">
  // Normal message
  .msg {
    position: relative;
    width: 100%;
    height: fit-content;

    margin-bottom: 6px;
    padding-left: 42px;
    font-family: LocalInter;
    font-weight: 400;

    .profile-picture {
      position: absolute;
      left: 0;
      top: 0;
    }

    h3 {
      height: 24px;
      color: #ddd;
      font-weight: 400;
      font-size: 14px;
      margin: 8px 0 0 0;

      span {
        font-size: 10px;
        color: #777;
        font-weight: 500;
        margin-left: 10px;
      }
    }

    .body {
      background-color: #282828;
      height: fit-content;
      width: fit-content;
      padding: 8px 12px 10px 12px;
      border-radius: 10px;

      p {
        height: fit-content;
        color: #ddd;
        font-size: 14px;
        white-space: pre-wrap;

        :global(a) {
          color: #6BA0D6;
        }
      }
    }
  }

  // Reaction header
  .reaction {
    position: relative;
    width: fit-content;
    height: 16px;

    display: flex;
    align-items: center;
    margin-top: 10px;

    &::before {
      content: "";

      position: absolute;
      top: 7px;
      left: 15px;

      width: 20px;
      height: 12px;

      border-left: 2px solid #4d4d4d;
      border-top: 2px solid #4d4d4d;

      border-top-left-radius: 10px;
    }

    .profile-picture {
      height: 16px;
      width: 16px;

      margin: 0 3px 0 42px;
    }

    h3 {
      height: 16px;
      font-size: 10px;
      font-weight: 600;
      color: #999;

      margin-right: 6px;
    }

    p {
      height: 16px;
      font-size: 10px;
      background-color: #323232;
      border-radius: 8px;
      padding: 2px 6px;

      :global(a) {
        color: rgba(143, 183, 223, 0.8);
      }
    }
  }

  // Thread message
  .thread {
    position: relative;
    width: 100%;
    height: 50px;

    margin-bottom: 6px;
    font-family: LocalInter;
    font-weight: 400;

    display: flex;
    align-items: center;

    > .profile-picture {
      margin: 0 8px 0 42px;
    }
    
    .body {
      background-color: #323232;
      height: 50px;
      width: 214px;
      border-radius: 10px;
      display: flex;
      flex-direction: column;

      p {
        width: calc(100% - 12px);
        height: fit-content;

        color: #999;
        font-size: 14px;

        margin: 8px 0 0 12px;
        display: flex;

        span {
          font-size: 10px;
          color: #777;
          font-weight: 500;

          margin: 2px 12px 0 auto;
        }
      }

      .footer {
        display: flex;
        align-items: center;

        .participants {
          display: flex;
          width: fit-content;
          margin: 0 0 0 10px;

          .profile-picture {
            margin-right: -8px;
            border: 3px solid #333;
            border-radius: 6px;
            background-color: #DC9552;
          }
        }

        .messages {
          font-size: 10px;
          color: #666;
          font-weight: 500;

          margin: 0 0 0 16px;

          .spacer {
            margin-left: 8px;
          }
        }
      }
    }
  }
</style>
