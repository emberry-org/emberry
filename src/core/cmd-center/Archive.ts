import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";
import { navigate } from 'svelte-navigator';

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools', input: false },

  { title: 'Close Window', type: CmdType.Control, accelerator: [ 'Alt', 'F4' ], action: () => { appWindow.close(); }, input: false },

  { title: 'Attempt Connection', type: CmdType.Debug, accelerator: [], action: (s: String) => { invoke('hole_punch', { peerKey: s }).then((id) => navigate('/chat/' + id)); }, input: true, input_desc: 'Enter peer public key...' },
];