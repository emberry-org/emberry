/**
 * Set the local users username.
 * @param username The new username.
 */
export function setUsername(username: string) {
  localStorage.setItem('username', username);
  dispatchEvent( new StorageEvent('storage', { key: 'username', storageArea: localStorage, newValue: username }) );
}

/**
 * Get the local users username.
 * @returns The username of our local user.
 */
export function getUsername(): string {
  return localStorage.getItem('username');
}

/**
 * Calls a callback whenever the tabs store is mutated.
 */
export function onUsernameChanged(callback: (username: string) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === localStorage && e.key === 'username') {
      callback(e.newValue);
    }
  });
}