export * from "./archive";
export * from "./cmd.type";

import { CmdArchive } from "./archive";
import type { CmdType, StringAction, VoidAction } from "./cmd.type";

/** Command interface */
export interface Cmd {
  title: string;
  type: CmdType;
  /** Also called a shortcut */
  accelerator: string[];
  /** The action to be executed */
  action: String | VoidAction | StringAction;
  /** Whether this actions takes input */
  input: boolean;
  /** Description of what this input is for */
  input_desc?: string;
}

/** Fetch commands from the archive using a query string */
export function fetch(query: string, maxResults: number = 10): Cmd[] {

  const results: Cmd[] = [];
  query = query.toLowerCase();

  /* First pass */
  for (let i = 0; i < CmdArchive.length; i++) {
    if (CmdArchive[i].title.toLowerCase().includes(query)) {
      results.push(CmdArchive[i]);
    }
    if (results.length >= maxResults) break;
  }

  return results;
}