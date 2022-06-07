import type Cmd from "./Cmd";
import { CmdType } from "./Cmd";

/** Collection of all the executable commands */
export const CmdArchive: Cmd[] = [
  { title: 'Toggle Developer Tools', type: CmdType.Developer, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools' },
  { title: 'Toggle Developer Tools', type: CmdType.Developer, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools' },
  { title: 'Toggle Developer Tools', type: CmdType.Developer, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools' },
  { title: 'Toggle Developer Tools', type: CmdType.Developer, accelerator: [ 'CmdOrCtrl', 'Shift', 'I' ], action: 'toggle_devtools' }
];