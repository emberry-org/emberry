import type AppTab from '@core/AppTab';
import type Msg from '@core/messages/Msg';
import { navigate } from 'svelte-navigator';
import { Store } from 'tauri-plugin-store-api';

let chatHistory: Store;
let appTabs: Store;

export function setupPersistentStore() {
  chatHistory = new Store('.chat-history.dat');
  appTabs = new Store('.app-tabs.dat');
}

/**
 * Insert a chat message into the chat history store.
 * @param chatId The ID of the chat.
 * @param msg The message to insert into the chat history.
 */
export async function insertChatHistory(chatId: string, msg: Msg) {
  let history = await getChatHistory(chatId);

  if (history) history.push(msg);
  else history = [msg];

  await chatHistory.set(chatId, history);
}

/**
 * Get the history of a specific chat.
 * @param chatId The ID of the chat.
 * @returns An array of messages.
 */
export async function getChatHistory(chatId: string): Promise<Array<Msg>> {
  return await chatHistory.get(chatId);
}

/**
 * Insert a tab into the application store.
 * @param tab The tab to insert.
 */
 export async function insertTab(tab: AppTab) {
  let tabs = await getTabs();

  if (tabs) tabs.push(tab);
  else tabs = [tab];

  await appTabs.set('tabs', tabs);
}

/**
 * Get the tabs stored in the application store.
 * @returns An array of application tabs.
 */
export async function getTabs(): Promise<Array<AppTab>> {
  return await appTabs.get('tabs');
}

/**
 * Calls a callback whenever the tabs store is mutated.
 */
export async function onTabsChange(callback: (tabs: Array<AppTab>) => void) {
  appTabs.onKeyChange('tabs', (value: Array<AppTab>) => {
    callback(value);
  });
}

/**
 * Close a tab with a given path.
 * @param path The path of the path to close.
 */
export async function closeTab(path: String) {
  let tabs = await getTabs();

  if (tabs) tabs = tabs.filter(tab => tab.path != path);
  else tabs = [];

  await appTabs.set('tabs', tabs);
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
export async function selectTab(path: String) {
  await appTabs.set('selected', path);
}

/**
 * Get the currently selected tab path.
 */
export async function getSelectedTab(): Promise<string> {
  return await appTabs.get('selected');
}

/**
 * Calls a callback whenever the selected tab is mutated.
 */
export async function onTabSelected(callback: (path: String) => void) {
  appTabs.onKeyChange('selected', (value: String) => {
    callback(value);
  });
}