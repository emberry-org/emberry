<script lang="ts">
  import Icon from 'svelte-icon/Icon.svelte';
  import { loadIcon } from '@core/utils/Icons';

  /** The name of the icon file without file extension */
  export let name: string;
  export let size: string = "20px";
  export let strokeWidth: string = "0em";

  $: name, fetchIcon(name);
  let icon: string = "";

  /** Fetch the icon svg with a given name */
  async function fetchIcon(name: string) {
    icon = await loadIcon(name);
  }
</script>

{#if icon}
  <Icon data={icon} size={size} stroke="currentColor" style="stroke-width: { strokeWidth }" />
{/if}