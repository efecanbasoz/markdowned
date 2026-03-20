import {
  lineNumbers,
  highlightActiveLine,
  highlightActiveLineGutter,
  EditorView,
  keymap,
} from "@codemirror/view";
import { closeBrackets } from "@codemirror/autocomplete";
import { history, defaultKeymap, historyKeymap } from "@codemirror/commands";
import { markdown } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import { bracketMatching, indentOnInput } from "@codemirror/language";
import { crispModernTheme, crispModernHighlight } from "./theme";
import { markdownKeybindings } from "./keybindings";

export function createExtensions(
  onUpdate: (content: string, line: number, col: number) => void
) {
  return [
    crispModernTheme,
    crispModernHighlight,
    lineNumbers(),
    highlightActiveLine(),
    highlightActiveLineGutter(),
    history(),
    closeBrackets(),
    bracketMatching(),
    indentOnInput(),
    markdown({ codeLanguages: languages }),
    markdownKeybindings,
    keymap.of([...defaultKeymap, ...historyKeymap]),
    EditorView.updateListener.of((update) => {
      if (update.docChanged || update.selectionSet) {
        const content = update.state.doc.toString();
        const cursor = update.state.selection.main.head;
        const line = update.state.doc.lineAt(cursor);
        onUpdate(content, line.number, cursor - line.from + 1);
      }
    }),
    EditorView.lineWrapping,
  ];
}
