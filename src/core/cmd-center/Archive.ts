import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";
import { clearChatHistory, insertTab, navigateTo } from '@store';

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  {
    title: '> Run', type: CmdType.Debug, accelerator: [], action: (s: String) =>
      invoke(s.toString()).then(err => console.log(`![${s}] -> `, err)),
    input: true
  },

  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: ['CmdOrCtrl', 'Shift', 'I'], action: 'toggle_devtools', input: false },

  { title: 'Close Window', type: CmdType.Control, accelerator: ['Alt', 'F4'], action: () => { appWindow.close(); }, input: false },

  { title: 'Connect Tls', type: CmdType.Debug, accelerator: [], action: 'connect', input: false },

  {
    title: 'Request Room', type: CmdType.Debug, accelerator: [], action: (s: string) => {
      let utf8Encode = new TextEncoder();
      invoke("request_room", { usr: { key: Array.from(utf8Encode.encode(s)) } }).then(err => console.log(`![${s}] -> `, err))
    },
    input: true
  },

  { title: 'Clear Chat History', type: CmdType.Debug, accelerator: [], action: () => { clearChatHistory(); }, input: false }
];