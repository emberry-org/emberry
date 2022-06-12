<script lang="ts">
  import Icon from 'svelte-icon/Icon.svelte';
  import { IconArchive } from '@assets/icons/Archive';

  /** The name of the icon file without file extension */
  export let name: string;
  export let size: string = "20px";
  export let strokeWidth: string = "0em";

  $: icon = fetchIcon(name);

  /** Fetch the icon svg with a given name */
  function fetchIcon(name: string): string {
    if (Object.keys(IconArchive).includes(name)) {
      return IconArchive[name];
    } else {
      return IconArchive["fallback"];
    }
  }
</script>

{#if icon}
  <Icon data={icon} size={size} stroke="currentColor" style="stroke-width: { strokeWidth }" />
{/if}