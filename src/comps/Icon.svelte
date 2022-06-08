<script lang="ts">
  import Icon from 'svelte-icon/Icon.svelte';
  import { IconArchive } from '../assets/icons/Archive';

  /** The name of the icon file without file extension */
  export let name: string;
  export let size: string = "20px";
  export let strokeWidth: string = "0em";

  $: icon = IconArchive[name];
</script>

{#if icon}
  <Icon data={icon} size={size} stroke="currentColor" fill="" style="stroke-width: { strokeWidth }" />
{/if}