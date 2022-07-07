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
 * Calls a callback whenever the username is mutated.
 */
export function onUsernameChanged(callback: (username: string) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === localStorage && e.key === 'username') {
      callback(e.newValue);
    }
  });
}

/**
 * Set the local users profile picture.
 * @param base64 The new profile picture encoded as base64.
 */
export function setProfilePicture(base64: string) {
  localStorage.setItem('profile_picture', base64);
  dispatchEvent( new StorageEvent('storage', { key: 'profile_picture', storageArea: localStorage, newValue: base64 }) );
}

/**
 * Get the local users profile picture.
 * @returns The profile picture of our local user encoded as base64.
 */
export function getProfilePicture(): string {
  return localStorage.getItem('profile_picture');
}

/**
 * Calls a callback whenever the profile picture is mutated.
 */
export function onProfilePictureChanged(callback: (base64: string) => void) {
  addEventListener('storage', e => {
    if (e.storageArea === localStorage && e.key === 'profile_picture') {
      callback(e.newValue);
    }
  });
}