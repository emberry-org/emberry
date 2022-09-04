<!-- License: https://github.com/sveltejs/svelte-virtual-list/blob/master/LICENSE -->

<script lang="ts">
	import { onMount, tick } from 'svelte';
import { dataset_dev } from 'svelte/internal';


	export let items: any[];
	export let height = '100%';
	export let itemHeight: number = undefined;
  export let reverse: boolean = false;

	// An array of all the heights of the elements.
	let heights: Array<number> = [];
	let elements: HTMLCollectionOf<HTMLElement>;
  
  // The viewport and list elements.
	let viewport: HTMLDivElement;
	let list: HTMLOListElement;
	let viewportHeight = 0, average_height: number;
	
	let mounted: boolean;
  let setup: boolean;

  // Offset from the top and bottom of the list.
	let topOffset = 0, bottomOffset = 0;
  let start = 0, end = 0;

  /** An array of the currently visible items. */
  let visible: any[];
	$: visible = items.slice(start, end).map((data, i) => {
		return { index: i + start, data };
	});

	// Refresh if the items array is updated.
	$: if (mounted) refresh(items, viewportHeight, itemHeight);

  /**
   * Refresh the current list render.
   * @param items The items to render.
   * @param viewportHeight The height of the viewport.
   * @param itemHeight The height of each item.
   */
	async function refresh(items: any[], viewportHeight: number, itemHeight: number) {
		const { scrollTop } = viewport;

    // Wait a tick for DOM
		await tick();

		let content_height = topOffset - scrollTop;
		let i = start;

    // Loop until we exceed the viewport size.
		while (content_height < viewportHeight && i < items.length) {
			let row = elements[i - start];

			if (!row) {
				end = i + 1; await tick(); // Wait a tick for DOM
				row = elements[i - start];
			}

			const row_height = heights[i] = itemHeight || row.offsetHeight;
			content_height += row_height;
			i += 1;
		}

		end = i;

    // Save the remaining height.
		const remaining = items.length - end;
		average_height = (topOffset + content_height) / end;

		bottomOffset = remaining * average_height;
		heights.length = items.length;

    if (!setup && reverse) {
      setup = true;
      await tick(); // Wait a tick for DOM
      viewport.scrollTop = viewport.scrollHeight;
      await refresh(items, viewportHeight, itemHeight);
    }
	}

  /**
   * Called when the user scrolls within the list.
   */
	async function onScroll() {
		const { scrollTop } = viewport;
		const old_start = start;

    // Save the height of each visible element.
		for (let v = 0; v < elements.length; v += 1) {
			heights[start + v] = itemHeight || elements[v].offsetHeight;
		}

		let i = 0;
		let y = 0;

    // Update the start and topOffset.
		while (i < items.length) {
			const row_height = heights[i] || average_height;
			if (y + row_height > scrollTop) {
				start = i; topOffset = y; break;
			}

			y += row_height; i += 1;
		}

    // Count from the start of the visible items to the end of the visible items.
		while (i < items.length) {
			y += heights[i] || average_height;
			i += 1;

			if (y > scrollTop + viewportHeight) break;
		}
		end = i;

    // Save the remaining height.
		const remaining = items.length - end;
		average_height = y / end;

    // Update the bottomOffset.
		while (i < items.length) heights[i++] = average_height;
		bottomOffset = remaining * average_height;

		// Prevent jumping if we scrolled up into unknown territory.
		if (start < old_start) {
			await tick(); // Wait a tick for DOM

			let expected_height = 0;
			let actual_height = 0;

			for (let i = start; i < old_start; i +=1) {
				if (elements[i - start]) {
					expected_height += heights[i];
					actual_height += itemHeight || elements[i - start].offsetHeight;
				}
			}

			const d = actual_height - expected_height;
			viewport.scrollTo(0, scrollTop + d);
		}

		// TODO if we overestimated the space these
		// elements would occupy we may need to add some
		// more. maybe we can just call onScroll again?
	}

	// trigger initial refresh
	onMount(async () => {
		elements = list.getElementsByTagName('li');
		mounted = true;
	});
</script>

<div class="s-list"
	style="height: { height };"

  on:scroll={ onScroll }

  bind:offsetHeight={ viewportHeight }
  bind:this={ viewport }
>
	<ol style="padding-top: { topOffset }px; padding-bottom: { bottomOffset }px;" bind:this={ list }>

    <li class="spacer" />
		{#each visible as row (row.index)}
    <li>
      <slot item={ row.data }>Missing template</slot>
    </li>
		{/each}

  </ol>
</div>

<style>
	.s-list {
		position: relative;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		display: block;
	}

  .s-list::-webkit-scrollbar {
    width: 0px;
  }

	ol, li {
		display: block;
	}

  ol {
    padding: 0 20px 0 20px;
  }

  ol .spacer {
    min-height: calc(100vh - 145px);    
  }

	li {
		overflow: hidden;
	}
</style>