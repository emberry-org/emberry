import type AppTab from '@core/AppTab';
import { writable } from 'svelte/store';

/** State of the command center (Whether its open or not) */
export const commandCenterState = writable<boolean>(false);

/** State of the address book (Whether its open or not) */
export const addressBookState = writable<boolean>(false);

/** Application tabs array displayed in the titlebar */
export const applicationTabs = writable<Array<AppTab>>([]);