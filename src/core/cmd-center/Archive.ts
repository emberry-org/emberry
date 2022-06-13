import { appWindow } from '@tauri-apps/api/window'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools', input: false },

  { title: 'Close Window', type: CmdType.Control, accelerator: [ 'Alt', 'F4' ], action: () => { /* appWindow.close(); */ }, input: false },

  { title: 'Input Test', type: CmdType.Control, accelerator: [ 'Alt', 'F4' ], action: (s: String) => { console.log(s); }, input: true },
];