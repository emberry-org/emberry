import { appWindow } from '@tauri-apps/api/window'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools' },

  { title: 'Close Window', type: CmdType.Window, accelerator: [ 'Alt', 'F4' ], action: () => { appWindow.close(); } },
];