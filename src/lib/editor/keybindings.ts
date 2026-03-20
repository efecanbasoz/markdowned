import { keymap } from "@codemirror/view";
import type { Command } from "@codemirror/view";

function wrapSelection(char: string): Command {
  return (view) => {
    const { from, to } = view.state.selection.main;
    if (from === to) return false;
    view.dispatch({
      changes: [
        { from, insert: char },
        { from: to, insert: char },
      ],
      selection: { anchor: from + char.length, head: to + char.length },
    });
    return true;
  };
}

const toggleBold: Command = wrapSelection("**");
const toggleItalic: Command = wrapSelection("*");

const insertLink: Command = (view) => {
  const { from, to } = view.state.selection.main;
  const selected = view.state.sliceDoc(from, to);
  const linkText = selected || "text";
  const replacement = `[${linkText}](url)`;
  view.dispatch({ changes: { from, to, insert: replacement } });
  return true;
};

export const markdownKeybindings = keymap.of([
  { key: "Mod-b", run: toggleBold },
  { key: "Mod-i", run: toggleItalic },
  { key: "Mod-Shift-k", run: insertLink },
]);
