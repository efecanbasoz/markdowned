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
import { Compartment } from "@codemirror/state";
import {
  crispModernTheme,
  crispModernHighlight,
  crispModernLightTheme,
  crispModernLightHighlight,
} from "./theme";
import { markdownKeybindings } from "./keybindings";
import { ghostTextExtension } from "./ghost-text";

export const themeCompartment = new Compartment();
const highlightCompartment = new Compartment();

function getThemeExtensions(isDark: boolean) {
  return isDark
    ? [crispModernTheme, crispModernHighlight]
    : [crispModernLightTheme, crispModernLightHighlight];
}

export function reconfigureTheme(view: EditorView, isDark: boolean) {
  const [theme, highlight] = getThemeExtensions(isDark);
  view.dispatch({
    effects: [
      themeCompartment.reconfigure(theme),
      highlightCompartment.reconfigure(highlight),
    ],
  });
}

export function createExtensions(
  onUpdate: (content: string, line: number, col: number, docChanged: boolean) => void,
  onTriggerCompletion?: (view: EditorView) => void,
  isDark: boolean = true
) {
  const [theme, highlight] = getThemeExtensions(isDark);
  return [
    themeCompartment.of(theme),
    highlightCompartment.of(highlight),
    lineNumbers(),
    highlightActiveLine(),
    highlightActiveLineGutter(),
    history(),
    closeBrackets(),
    bracketMatching(),
    indentOnInput(),
    markdown({ codeLanguages: languages }),
    markdownKeybindings,
    ghostTextExtension,
    keymap.of([
      ...defaultKeymap,
      ...historyKeymap,
      ...(onTriggerCompletion
        ? [
            {
              key: "Mod-Enter",
              run: (view: EditorView) => {
                onTriggerCompletion(view);
                return true;
              },
            },
          ]
        : []),
    ]),
    EditorView.updateListener.of((update) => {
      if (update.docChanged || update.selectionSet) {
        const content = update.state.doc.toString();
        const cursor = update.state.selection.main.head;
        const line = update.state.doc.lineAt(cursor);
        onUpdate(content, line.number, cursor - line.from + 1, update.docChanged);
      }
    }),
    EditorView.lineWrapping,
  ];
}
