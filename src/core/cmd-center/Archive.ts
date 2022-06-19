import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";
import { insertTab, navigateTo } from '@store';

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Debug, accelerator: ['CmdOrCtrl', 'Shift', 'I'], action: 'toggle_devtools', input: false },

  { title: 'Close Window', type: CmdType.Control, accelerator: ['Alt', 'F4'], action: () => { appWindow.close(); }, input: false },

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
];