<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import type Cmd from "@core/cmd-center/Cmd";
  import type { StringAction,VoidAction } from "@core/cmd-center/Cmd";
  import fetch from "@core/cmd-center/CmdFetcher";
  import { onDestroy, onMount } from "svelte";
  import { commandCenterState } from "@store";
  import Icon from '@lib/Icon.svelte';

  let panel: HTMLDivElement;
  let input: HTMLInputElement;

  let commands: Cmd[] = [];
  let searchString: string = "";
  let selected: number = 0;

  let selectedAction: StringAction;
  let inputMode: boolean = false;
  let placeholderText: string = "Search commands by name (not case sensitive)";

  onMount(() => {
    /* Check if the user clicks outside the panel */
    document.addEventListener("mousedown", onMouseDown, true);
    document.addEventListener("keydown", onKeyDown, true);

    /* Focus the input field */
    input.focus();

    onInputChanged();
  });

  onDestroy(() => {
    /* Remove the event listener after the element is destroyed */
    document.removeEventListener("mousedown", onMouseDown, true);
    document.removeEventListener("keydown", onKeyDown, true);
  });

  /** Called when the user clicks somewhere in the app */
  const onMouseDown = (e: MouseEvent) => {
    const bounds: DOMRect = panel.getBoundingClientRect();

    /* Close the command center if the users clicks outside of its bounds */
    if (e.x <= bounds.left || e.x >= bounds.right || e.y <= bounds.top || e.y >= bounds.bottom) {
      commandCenterState.set(false);
    }
  };

  /** Called when the user inputs a key */
  const onKeyDown = (e: KeyboardEvent) => {

    if (inputMode) {
      if (e.key == 'Enter') {
        e.preventDefault();
        selectedAction(searchString);
        commandCenterState.set(false);
        inputMode = false;
      }

      return;
    }

    if (e.key == 'ArrowDown') {
      e.preventDefault();
      selected = Math.min(selected + 1, commands.length - 1);
    }

    if (e.key == 'ArrowUp') {
      e.preventDefault();
      selected = Math.max(selected - 1, 0);
    }

    if (e.key == 'Enter') {
      e.preventDefault();
      if (selected >= 0 && selected < commands.length && commands.length > 0) {
        executeCommand(commands[selected]);
      }
    }
  };

  /** Called when the search input field is changed */
  const onInputChanged = () => {
    if (!inputMode) {
      commands = fetch(searchString);
    }
  };

  /** Execute the command */
  const executeCommand = (cmd: Cmd) => {
    let backendAction = cmd.action as string;
    let stringAction = cmd.action as StringAction;
    let voidAction = cmd.action as VoidAction;

    if (typeof cmd.action == 'string') {
      invoke(backendAction);
      commandCenterState.set(false);
      inputMode = false;
    } else if (cmd.input == true) {
      selectedAction = stringAction;
      if (cmd.input_desc) placeholderText = cmd.input_desc;
      commands = [cmd];
      searchString = "";
      inputMode = true;
    } else if (typeof cmd.action == 'function') {
      voidAction();
      commandCenterState.set(false);
      inputMode = false;
    } else {
      console.error("Unknown command action");
      commandCenterState.set(false);
      inputMode = false;
    }
  };
</script>

<div class="command-center" bind:this={panel}>

  <input type="text" class="input" 
    placeholder={placeholderText} 
    bind:this={input} bind:value={searchString} on:input={onInputChanged}
  >

  <div class="suggestions">
    {#each commands as cmd, i}
      <div class="command { selected == i ? 'selected' : '' }" on:click={() => executeCommand(cmd)}>
        <div class="icon"> <Icon name={ cmd.type } size="20px" /> </div>
        <div class="title"> { cmd.title } </div>
        <div class="shortcut">
          {#each cmd.accelerator as part, j}
            <span> { part } </span>
            {#if j < cmd.accelerator.length - 1} + {/if}
          {/each}
        </div>
      </div>
    {/each}   
  </div>

</div>

<style lang="scss">

.command-center {
  position: absolute;

  width: 60vw;
  max-width: 420px;
  height: fit-content;
  max-height: 60vh;

  top: 4px;
  left: 50%;

  transform: translateX(-50%);

  background-color: #181a19;
  border: 1.5px solid #ffffff22;
  border-radius: 6px;

  overflow: hidden;
  display: flex;
  flex-direction: column;
  z-index: 9999;

  .input {
    width: 100%;
    height: 36px;

    padding-left: 12px;

    outline: none;
    border: none;
    border-bottom: 1.5px solid #ffffff1b;
    background: #181a19;
    color: #dddddd;

    font-size: 0.9rem;
  }

  .suggestions {
    flex-grow: 1;
    width: 100%;
    font-family: Inter;

    .command {
      width: 100%;
      height: 28px;
      display: flex;
      align-items: center;

      cursor: pointer;

      &:hover, &.selected {
        background-color: #242625;

        .icon {
          color: #aaa;
        }

        .title {
          color: #aaa;
        }
      }

      .icon {
        width: 20px;
        height: 20px;

        margin-left: 8px;
        margin-right: 8px;

        color: #4d4d4d;

        display: flex;
        justify-content: center;
        align-items: center;
      }

      .title {
        color: #6d6d6d;
        font-size: 13px;
        font-weight: 450;
        flex-grow: 1;

        white-space: nowrap;
      }

      .shortcut {
        font-size: 13px;
        color: #555555;
        margin-right: 14px;
        min-width: fit-content;

        span {
          font-size: 11px;
          padding: 2px 6px 2px 6px;
          background-color: #ffffff11;
          color: #999999;
          border-radius: 4px;
          margin-left: 5px;
        }
      }
    }
  }
}

</style>