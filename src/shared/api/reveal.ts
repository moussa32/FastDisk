import { revealInExplorer } from "./commands";
import type { RevealInExplorerInput } from "./types";

export function revealPath(input: RevealInExplorerInput): Promise<void> {
  return revealInExplorer(input);
}
