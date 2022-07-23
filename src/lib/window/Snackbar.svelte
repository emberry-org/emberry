<script lang="ts">
  import Icon from "@lib/Icon.svelte";
  import type { Snack, SnackAction } from "@core/Snack";
  import { onMount } from "svelte";
  import { addSnack, closeSnack, getSnacks, onSnackBarChanged } from "@store";
import type { VoidAction } from "@core/cmd-center/Cmd";

  let snacks: Snack[] = [];

  onMount(() => {
    snacks = getSnacks();

    addSnack({
      title: "Welcome to Tauri!",
      description: "This is a simple chat app built with Tauri.",
      actions: [
        {
          label: "Thanks",
          handler: () => {
            console.log("Thanks!");
          },
          class: "positive"
        },
        {
          label: "Decline",
          handler: () => {
            console.log("Declined!");
          },
        }
      ]
    });

    onSnackBarChanged(newSnacks => {
      snacks = newSnacks;
    });
  });

  const invokeAction = (action: SnackAction) => {
    const func = action.handler as VoidAction;
    func();
  };

  const removeSnack = (i: number) => {
    closeSnack(i);
    snacks.splice(i, 1);
    snacks = [...snacks];
  };
</script>

<div class="snackbar">
  {#each snacks as snack, i}
    <div class="snack">

      <header>
        <div class="title">{snack.title}</div>
        <button class="close" on:click={() => removeSnack(i)}>
          <Icon name="window/close" size="20px" />
        </button>
      </header>
      
      <div class="body">
        <div class="desc">{snack.description}</div>

        <div class="actions">
          {#each snack.actions as action}
            <button class="action { action.class }" on:click={() => { invokeAction(action); removeSnack(i); }}>{action.label}</button>
          {/each}
        </div>
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

    background-color: var(--ffg);
    border: 2px solid var(--tb);
    box-shadow: 0 8px 24px #1c1c1c;
    border-radius: 8px;

    display: flex;
    flex-direction: column;

    header {
      display: flex;
      align-items: center;

      padding-left: 8px;
      margin-bottom: 6px;

      background-color: var(--mg);
      border-top-left-radius: 6px;
      border-top-right-radius: 6px;
      
      .title {
        color: #ccc;
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
        color: #888;
  
        cursor: pointer;
  
        &:hover {
          color: #ddd;
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
        color: #aaa;
        margin-bottom: 3px;
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
