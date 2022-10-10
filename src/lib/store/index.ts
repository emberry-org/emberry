/**
 * Sets an item in the given storage and dispatches an event for that storage.
 * @param store The storage to mutate.
 * @param key The key of the item to set.
 * @param value The new value of the item.
 */
export function setItem(store: Storage, key: string, value: string) {
  store.setItem(key, value);

  // Dispatch a new storage event.
  dispatchEvent(
    new StorageEvent('storage', { 
      key, 
      storageArea: store, 
      newValue: value 
    }
  ));
}

/**
 * Get an item from the given storage.
 * @param store The storage to read.
 * @param key The key of the item to get.
 */
export function getItem(store: Storage, key: string): string | null {
  return store.getItem(key);
}

type StorageCallback = (value: string | null) => void;

/**
 * Add an event listener to an item in a given storage.
 * @param store The storage to listen.
 * @param callback A callback that is called whenever the item is mutated.
 * @param key The key of the item to listen for.
 * @returns The initial value of the item.
 */
export function onItem(store: Storage, callback: StorageCallback, key: string): string | null {
  // Listen for the storage update event.
  addEventListener('storage', e => {
    if (e.storageArea === store && e.key === key) {
      callback(e.newValue);
    }
  });

  // Return the initial value of the item.
  return store.getItem(key);
}