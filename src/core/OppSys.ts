import { oppSys } from '@store';
import { platform } from '@tauri-apps/api/os';
import { appWindow } from '@tauri-apps/api/window'

export type OsType = "linux" | "darwin" | "ios" | "freebsd" | "dragonfly" | "netbsd" | "openbsd" | "solaris" | "android" | "win32";

/**
 * Setup the opperating system specific things
 */
export default async function setupOS() {
  const os = await platform();

  switch (os) {
    // Windows
    case 'win32':
      appWindow.setDecorations(false);
      break;

    // Linux
    case 'linux':
      appWindow.setDecorations(true);
      break;

    // MacOS
    case 'darwin':
      appWindow.setDecorations(true);
      break;
  
    default:
      break;
  }

  oppSys.set(os);
}