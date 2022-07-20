import type AppTab from "@core/AppTab";
import type Drink from "@core/Drink";
import { navigate } from "svelte-navigator";

/**
 * Insert a drink into the application store.
 * @param drink The drink to insert.
 */
 export function addDrink(drink: Drink) {
  let drinks = getDrinks();

  if (drinks) drinks.push(drink);
  else drinks = [drink];

  const json = JSON.stringify(drinks);

  sessionStorage.setItem('drinks', json);
  dispatchEvent( new StorageEvent('storage', { key: 'drinks', storageArea: sessionStorage, newValue: json }) );
}

/**
 * Get the drinks stored in the application store.
 * @returns An array of application drinks.
 */
export function getDrinks(): Array<Drink> {
  return JSON.parse(sessionStorage.getItem('drinks'));
}

/**
 * Calls a callback whenever the drinks store is mutated.
 */
export function onDrinksChanged(callback: (drinks: Array<Drink>) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === sessionStorage && e.key === 'drinks') {
      callback(JSON.parse(e.newValue));
    }
  });
}


/**
 * Insert a tab into the application store.
 * @param tab The tab to insert.
 */
export function insertTab(tab: AppTab) {
  let tabs = getTabs();

  if (tabs) tabs.push(tab);
  else tabs = [tab];

  const json = JSON.stringify(tabs);

  sessionStorage.setItem('tabs', json);
  dispatchEvent( new StorageEvent('storage', { key: 'tabs', storageArea: sessionStorage, newValue: json }) );
}

/**
 * Get the tabs stored in the application store.
 * @returns An array of application tabs.
 */
export function getTabs(): Array<AppTab> {
  return JSON.parse(sessionStorage.getItem('tabs'));
}

/**
 * Calls a callback whenever the tabs store is mutated.
 */
export function onTabsChange(callback: (tabs: Array<AppTab>) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === sessionStorage && e.key === 'tabs') {
      callback(JSON.parse(e.newValue));
    }
  });
}

/**
 * Close a tab with a given path.
 * @param path The path of the path to close.
 */
export function closeTab(path: String) {
  let tabs = getTabs();

  if (tabs) tabs = tabs.filter(tab => tab.path != path);
  else tabs = [];

  const json = JSON.stringify(tabs);

  sessionStorage.setItem('tabs', json);
  dispatchEvent( new StorageEvent('storage', { key: 'tabs', storageArea: sessionStorage, newValue: json }) );
}

/**
 * Navigate to a given path. (MUST BE ABSOLUTE)
 * @param path 
 */
export async function navigateTo(path: string) {
  if (path.startsWith('/') == false) { console.error('Path for navigation must be absolute!'); return; }
  // Navigate to the given path.
  navigate(path, { replace: true });
  // Select the tab connected to this path.
  selectTab(path);
}

/**
 * Select a tab and store it in the store.
 * @param path The path of the tab to select.
 */
export function selectTab(path: string) {
  sessionStorage.setItem('selected_tab', path);
  dispatchEvent( new StorageEvent('storage', { key: 'selected_tab', storageArea: sessionStorage, newValue: path }) );
}

/**
 * Get the currently selected tab path.
 */
export function getSelectedTab(): string {
  return sessionStorage.getItem('selected_tab');
}

/**
 * Calls a callback whenever the selected tab is mutated.
 */
export function onTabSelected(callback: (path: string) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === sessionStorage && e.key === 'selected_tab') {
      callback(e.newValue);
    }
  });
}