<script lang="ts">
  import Icon from "@lib/Icon.svelte";
  import type { Snack } from "@core/Snack";
  import { onMount } from "svelte";
  import { addSnack, closeSnack, getSnacks, onSnackBarChanged } from "@store";
  import { emit, listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/tauri'

  let snacks: Snack[] = [];

  onMount(() => {
    snacks = getSnacks();

    onSnackBarChanged(newSnacks => {
      snacks = newSnacks;
    });

    // Setup the wants-room event listener.
    // TODO: Move this to a better place.
    listen('wants-room', (e: any) => {
      const usrkey = e.payload.key;

      addSnack({
        title: "Room Request",
        description: `${ usrkey } has send you a room request.`,
        actions: [
          {
            label: "Accept",
            class: "positive",

            isCommand: true,
            key: "accept_room",
            payload: {
              usr: { key: usrkey },
              accepted: true,
            }
          },
          {
            label: "Decline",

            isCommand: true,
            key: "accept_room",
            payload: {
              usr: { key: usrkey },
              accepted: false,
            }
          }
        ]
      });
    });
  });

  /** Invoke a snackbar action */
  const invokeAction = (key: string, isCommand?: boolean, payload?: any) => {

    if (isCommand === true) {
      invoke(key, payload);
    } else {
      emit(key, payload);
    }
  };

  const removeSnack = (i: number) => {
    closeSnack(i);
    snacks.splice(i, 1);
    snacks = [...snacks];
  };
</script>

<div class="snackbar">
  {#each snacks.reverse() as snack, i}
    <div class="snack">

      <header>
        <Icon name="notifications/connect" size="16px" />
        <div class="title">{snack.title}</div>
        <button class="close" on:click={() => removeSnack(snacks.length - i - 1)}>
          <Icon name="navigation/close" size="20px" />
        </button>
      </header>
      
      <div class="body">
        <div class="desc">{snack.description}</div>

        {#if snack.actions}
          <div class="actions">
            {#each snack.actions as action}
              <button class="action { action.class }" 
                on:click={() => { invokeAction(action.key, action.isCommand, action.payload); 
                removeSnack(snacks.length - i - 1); }}
              >
                {action.label}
              </button>
            {/each}
          </div>
        {/if}
      </div>

    </div>
  {/each}
</div>

<style lang="scss">

.snackbar {
  position: absolute;
  bottom: 0;
  right: 0;

  width: 380px;
  max-width: 50vw;
  height: 720px;
  max-height: 80vh;
  pointer-events: none;

  display: flex;
  flex-direction: column-reverse;
  align-items: flex-end;
  padding: 40px 16px;

  .snack {
    position: relative;
    width: fit-content;
    height: fit-content;
    pointer-events: all !important;
    z-index: 100;

    background-color: #332f2d;
    border: 2px solid #403d3a;
    box-shadow: 0 8px 24px #1c1c1c;
    border-radius: 8px;

    display: flex;
    flex-direction: column;
    margin-top: 24px;

    header {
      display: flex;
      align-items: center;

      padding-left: 8px;
      margin-bottom: 6px;

      background-color: #332f2d;
      border-bottom: 2px solid #403d3a;
      border-top-left-radius: 6px;
      border-top-right-radius: 6px;

      > :global(svg) {
        color: #63c466;
        margin-right: 8px;
      }
      
      .title {
        color: #cfc9c6;
        font-size: 14px;
      }

      .close {
        width: 32px;
        height: 32px;
  
        display: flex;
        justify-content: center;
        align-items: center;
  
        margin-left: auto;
  
        background-color: #0000;
        border: none;
        outline: none;
        color: #504d4a;
  
        cursor: pointer;
  
        &:hover {
          color: #afa9a6;
        }
      }
    }

    .body {
      display: flex;
      flex-direction: column;
      padding: 0 10px 0 8px;
      width: calc(100% - 18px);

      .desc {
        font-size: 14px;
        color: #afa9a6;
        margin-bottom: 6px;
      }

      .actions {
        display: flex;
        justify-content: space-around;
        padding: 5px 0;

        .action {
          width: 64px;
          height: 32px;

          background-color: #ffffff18;
          border: 2px solid #ffffff18;
          color: #ccc;

          border-radius: 4px;
          font-size: 14px;

          cursor: pointer;

          &.positive {
            background-color: #125488;
            border: 2px solid #176bae;
            color: #f2eee8;

            &:hover {
              background-color: #226498;
              border: 2px solid #277bbe;
            }
          }

          &:hover {
            background-color: #ffffff28;
            border: 2px solid #ffffff28;
          }
        }
      }
    }
  }
}

</style>
