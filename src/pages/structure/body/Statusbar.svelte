<script lang="ts">

  // TODO: The footer will be static for now, the goal is for it to be dynamic in the future.

  import Icon from '@lib/Icon.svelte';
  import { getTauriVersion, getVersion } from '@tauri-apps/api/app';
  import { platform } from '@tauri-apps/api/os';

  let tauriVersion = '0.0.0';
  let appVersion = '0.0.0';
  let platformName = 'unknown';

  getTauriVersion().then((v) => tauriVersion = v);
  getVersion().then((v) => appVersion = v);
  platform().then((p) => platformName = p);

</script>

<footer class="statusbar">
  <div class="left-side">
    <div class="item purple">
      <p>Emberry v{ appVersion }</p>
    </div>

    <div class="item">
      <Icon name="brand/tauri" size="12px" />
      <p>Tauri v{ tauriVersion }</p>
    </div>
  </div>

  <div class="right-side">
    <div class="item">
      <Icon name="app/network" size="14px" />
      <p>12ms</p>
    </div>
  </div>
</footer>

<style lang="scss">

.statusbar {
  width: 100%;
  height: 24px;
  min-height: 24px;

  display: flex;

  background-color: #1f2022;

  user-select: none;
  -webkit-user-select: none;
  
  .left-side {
    width: 50%;
    height: 100%;

    margin-left: 72px;
    border-left: 1.5px solid var(--tb);

    display: flex;
  }

  .right-side {
    width: 50%;
    height: 100%;

    display: flex;
    justify-content: flex-end;
    align-items: center;
  }

  .item {
    width: fit-content;
    height: calc(100% - 1px);

    display: flex;
    align-items: center;

    padding: 0 12px 1px 12px;
    color: #fff6;

    :global(svg) {
      margin-right: 6px;
      margin-bottom: 2px;
    }

    p {
      color: #7b7d81;
      font-size: 12px;

      line-height: 12px;
    }
  }

  .purple {
    background-color: #273628;
    border-top-right-radius: 6px;

    p {
      color: #7e9781;
    }
  }
}

</style>