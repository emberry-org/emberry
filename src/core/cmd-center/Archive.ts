import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools', input: false },

  { title: 'Close Window', type: CmdType.Control, accelerator: [ 'Alt', 'F4' ], action: () => { appWindow.close(); }, input: false },

  { title: 'Create / Join Room', type: CmdType.Debug, accelerator: [], action: (s: String) => { console.log('creating / joining room: ', s); invoke('hole_punch', { ident: s }).then((r) => console.log(r)); }, input: true },
];