import { writable } from 'svelte/store';

/** State of the command center (Whether its open or not) */
export const commandCenterState = writable<boolean>(false);