import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

import type { Cmd } from "./cmd";
import { CmdType } from './cmd.type';

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  {
    title: '> Run', type: CmdType.Debug, accelerator: [], action: (s: String) =>
      invoke(s.toString()).then(err => console.log(`![${s}] -> `, err)),
    input: true
  },

  { title: 'Close Window', type: CmdType.Control, accelerator: ['Alt', 'F4'], action: () => { appWindow.close(); }, input: false },
];