import {
  EditorView,
  Decoration,
  type DecorationSet,
  WidgetType,
  keymap,
  type Command,
} from "@codemirror/view";
import { StateField, StateEffect } from "@codemirror/state";

export const setGhostText = StateEffect.define<string>();
export const appendGhostText = StateEffect.define<string>();
export const clearGhostText = StateEffect.define<void>();

class GhostTextWidget extends WidgetType {
  constructor(readonly text: string) {
    super();
  }

  toDOM() {
    const span = document.createElement("span");
    span.className = "cm-ghost-text";
    span.textContent = this.text;
    span.style.color = "var(--text-dimmed, #6b7280)";
    span.style.fontStyle = "italic";
    span.style.opacity = "0.5";
    return span;
  }

  eq(other: GhostTextWidget) {
    return this.text === other.text;
  }
}

/**
 * Extract the current ghost text content from a DecorationSet by
 * iterating its decorations and reading the widget text.
 */
function getGhostContent(decos: DecorationSet): string | null {
  let text: string | null = null;
  const cursor = decos.iter();
  while (cursor.value) {
    const widget = cursor.value.spec.widget;
    if (widget instanceof GhostTextWidget) {
      text = widget.text;
      break;
    }
    cursor.next();
  }
  return text;
}

function makeGhostDeco(pos: number, text: string): DecorationSet {
  const deco = Decoration.widget({
    widget: new GhostTextWidget(text),
    side: 1,
  });
  return Decoration.set([deco.range(pos)]);
}

export const ghostTextField: StateField<DecorationSet> = StateField.define<DecorationSet>({
  create() {
    return Decoration.none;
  },

  update(value, tr) {
    // If the document changed (user typing), dismiss ghost text
    if (tr.docChanged) {
      return Decoration.none;
    }

    for (const effect of tr.effects) {
      if (effect.is(setGhostText)) {
        const pos = tr.state.selection.main.head;
        return makeGhostDeco(pos, effect.value);
      }
      if (effect.is(appendGhostText)) {
        const existing = getGhostContent(value);
        const newText = (existing ?? "") + effect.value;
        const pos = tr.state.selection.main.head;
        return makeGhostDeco(pos, newText);
      }
      if (effect.is(clearGhostText)) {
        return Decoration.none;
      }
    }

    return value;
  },

  provide(field) {
    return EditorView.decorations.from(field);
  },
});

/**
 * Command: accept ghost text by inserting it at cursor position.
 * Returns true if ghost text was accepted, false if there was none.
 */
export const acceptGhostText: Command = (view) => {
  const decos = view.state.field(ghostTextField);
  const text = getGhostContent(decos);
  if (!text) return false;

  const pos = view.state.selection.main.head;
  view.dispatch({
    changes: { from: pos, insert: text },
    selection: { anchor: pos + text.length },
    effects: clearGhostText.of(undefined),
  });
  return true;
};

const dismissGhostText: Command = (view) => {
  const decos = view.state.field(ghostTextField);
  const text = getGhostContent(decos);
  if (!text) return false;

  view.dispatch({ effects: clearGhostText.of(undefined) });
  return true;
};

export const ghostTextExtension = [
  ghostTextField,
  keymap.of([
    { key: "Tab", run: acceptGhostText },
    { key: "Escape", run: dismissGhostText },
  ]),
];
