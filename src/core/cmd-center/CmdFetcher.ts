import { CmdArchive } from "./Archive";
import type Cmd from "./Cmd";

/** Fetch commands from the archive using a query string */
export default function fetch(query: string, maxResults: number = 10): Cmd[] {

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