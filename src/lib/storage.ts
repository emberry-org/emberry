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
            newValue: value
        }
    ));
}

/**
 * Get an item from the given storage.
 * @param store The storage to read.
 * @param key The key of the item to get.
 */
export function getItem(store: Storage, key: string): string | undefined {
    return store.getItem(key);
}

/**
 * Updates an item in the given storage and dispatches an event for that storage.
 * @param store The storage to mutate.
 * @param key The key of the item to update.
 * @param mutator Function for updating the item.
 */
export function updateItem<T>(store: Storage, key: string, mutator: (value: T) => T) {
    const old_value: T = JSON.parse(store.getItem(key));
    const new_value: string = JSON.stringify(mutator(old_value));

    store.setItem(key, new_value);

    // Dispatch a new storage event.
    dispatchEvent(
        new StorageEvent('storage', {
            key,
            newValue: new_value
        }
    ));
}

type StorageCallback = (value: any) => void;

/**
 * Add an event listener to an item in a given storage.
 * @param key The key of the item to listen for.
 * @param callback A callback that is called whenever the item is mutated.
 */
export function onItem(key: string, callback: StorageCallback) {
    // Listen for the storage update event.
    addEventListener('storage', e => {
        if (e.key === key) {
            callback(JSON.parse(e.newValue));
        }
    });
}
