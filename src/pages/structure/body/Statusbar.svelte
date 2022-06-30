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
    <div class="item">
      <Icon name="app/version" size="16px" />
      <p>v{ appVersion }</p>
    </div>

    <div class="item">
      <Icon name="brand/tauri" size="12px" />
      <p>v{ tauriVersion }</p>
    </div>
  </div>

  <div class="right-side">
    <div class="item">
      <p>{ platformName }</p>
    </div>
  </div>
</footer>

<style lang="scss">

.statusbar {
  width: 100%;
  height: 24px;
  min-height: 24px;

  display: flex;

  background-color: #212327;
  border-left: 1.5px solid #ffffff16;

  user-select: none;
  -webkit-user-select: none;
  
  .left-side {
    width: 50%;
    height: 100%;

    display: flex;
  }

  .right-side {
    width: 50%;
    height: 100%;

    padding-right: 7px;

    display: flex;
    justify-content: flex-end;
    align-items: center;
  }

  .item {
    width: fit-content;
    height: 100%;

    display: flex;
    align-items: center;

    margin: 0 6px 1px 6px;
    color: #888;

    p {
      color: #888;
      font-size: 12px;

      margin-left: 3px;
    }
  }
}

</style>