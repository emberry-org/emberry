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
    title: 'Request Room', type: CmdType.Debug, accelerator: [], action: (s: String) =>
      invoke("request_room", { user: { key: s } }).then(err => console.log(`![${s}] -> `, err)),
    input: true
  },

  {
    title: 'Attempt Connection', type: CmdType.Debug, accelerator: [], action: (s: String) => {

      invoke('hole_punch', { peerKey: s }).then((id: String) => {
        // Create a new tab once the chat has been created.
        insertTab({ icon: 'chat', title: id.substring(0, 6), path: '/chat/' + id, keep_open: true });
        // Navigate to the chat tab.
        navigateTo('/chat/' + id);
      });
    },

    input: true, input_desc: 'Enter peer public key...'
  },

  { title: 'Clear Chat History', type: CmdType.Debug, accelerator: [], action: () => { clearChatHistory(); }, input: false }
];