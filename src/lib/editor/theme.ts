import { EditorView } from "@codemirror/view";
import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { tags } from "@lezer/highlight";

// --- Dark Theme ---

export const crispModernTheme = EditorView.theme(
  {
    "&": { backgroundColor: "#0f1117", color: "#d1d5db", height: "100%" },
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', 'SF Mono', monospace",
      fontSize: "13px",
      lineHeight: "1.8",
      padding: "16px 20px",
      caretColor: "#60a5fa",
    },
    ".cm-cursor": { borderLeftColor: "#60a5fa", borderLeftWidth: "2px" },
    ".cm-gutters": {
      backgroundColor: "#0f1117",
      color: "#2d3148",
      border: "none",
      paddingRight: "8px",
    },
    ".cm-activeLineGutter": { color: "#4b5168", backgroundColor: "transparent" },
    ".cm-activeLine": { backgroundColor: "rgba(37, 99, 235, 0.05)" },
    ".cm-selectionBackground": {
      backgroundColor: "rgba(37, 99, 235, 0.2) !important",
    },
    "&.cm-focused .cm-selectionBackground": {
      backgroundColor: "rgba(37, 99, 235, 0.3) !important",
    },
    ".cm-line": { padding: "0 4px" },
  },
  { dark: true }
);

export const crispModernHighlight = syntaxHighlighting(
  HighlightStyle.define([
    { tag: tags.heading1, color: "#60a5fa", fontSize: "1.3em", fontWeight: "700" },
    { tag: tags.heading2, color: "#60a5fa", fontSize: "1.15em", fontWeight: "600" },
    { tag: tags.heading3, color: "#60a5fa", fontSize: "1.05em", fontWeight: "600" },
    { tag: [tags.heading4, tags.heading5, tags.heading6], color: "#60a5fa", fontWeight: "600" },
    { tag: tags.strong, color: "#f3f4f6", fontWeight: "600" },
    { tag: tags.emphasis, color: "#f3f4f6", fontStyle: "italic" },
    { tag: tags.strikethrough, color: "#6b7280", textDecoration: "line-through" },
    { tag: tags.link, color: "#60a5fa", textDecoration: "underline" },
    { tag: tags.url, color: "#60a5fa" },
    { tag: tags.monospace, color: "#34d399", backgroundColor: "rgba(52, 211, 153, 0.1)" },
    { tag: tags.keyword, color: "#c084fc" },
    { tag: tags.string, color: "#34d399" },
    { tag: tags.number, color: "#e8a84c" },
    { tag: tags.comment, color: "#4b5168", fontStyle: "italic" },
    { tag: tags.function(tags.variableName), color: "#60a5fa" },
    { tag: tags.typeName, color: "#60a5fa" },
    { tag: tags.meta, color: "#4b5168" },
    { tag: tags.processingInstruction, color: "#4b5168" },
  ])
);

// --- Light Theme ---

export const crispModernLightTheme = EditorView.theme(
  {
    "&": { backgroundColor: "#ffffff", color: "#1a1a2e", height: "100%" },
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', 'SF Mono', monospace",
      fontSize: "13px",
      lineHeight: "1.8",
      padding: "16px 20px",
      caretColor: "#2563eb",
    },
    ".cm-cursor": { borderLeftColor: "#2563eb", borderLeftWidth: "2px" },
    ".cm-gutters": {
      backgroundColor: "#ffffff",
      color: "#b0b0c0",
      border: "none",
      paddingRight: "8px",
    },
    ".cm-activeLineGutter": { color: "#8888a0", backgroundColor: "transparent" },
    ".cm-activeLine": { backgroundColor: "rgba(37, 99, 235, 0.06)" },
    ".cm-selectionBackground": {
      backgroundColor: "rgba(37, 99, 235, 0.15) !important",
    },
    "&.cm-focused .cm-selectionBackground": {
      backgroundColor: "rgba(37, 99, 235, 0.25) !important",
    },
    ".cm-line": { padding: "0 4px" },
  },
  { dark: false }
);

export const crispModernLightHighlight = syntaxHighlighting(
  HighlightStyle.define([
    { tag: tags.heading1, color: "#2563eb", fontSize: "1.3em", fontWeight: "700" },
    { tag: tags.heading2, color: "#2563eb", fontSize: "1.15em", fontWeight: "600" },
    { tag: tags.heading3, color: "#2563eb", fontSize: "1.05em", fontWeight: "600" },
    { tag: [tags.heading4, tags.heading5, tags.heading6], color: "#2563eb", fontWeight: "600" },
    { tag: tags.strong, color: "#1a1a2e", fontWeight: "600" },
    { tag: tags.emphasis, color: "#1a1a2e", fontStyle: "italic" },
    { tag: tags.strikethrough, color: "#8888a0", textDecoration: "line-through" },
    { tag: tags.link, color: "#2563eb", textDecoration: "underline" },
    { tag: tags.url, color: "#2563eb" },
    { tag: tags.monospace, color: "#059669", backgroundColor: "rgba(5, 150, 105, 0.08)" },
    { tag: tags.keyword, color: "#7c3aed" },
    { tag: tags.string, color: "#059669" },
    { tag: tags.number, color: "#d97706" },
    { tag: tags.comment, color: "#b0b0c0", fontStyle: "italic" },
    { tag: tags.function(tags.variableName), color: "#2563eb" },
    { tag: tags.typeName, color: "#2563eb" },
    { tag: tags.meta, color: "#b0b0c0" },
    { tag: tags.processingInstruction, color: "#b0b0c0" },
  ])
);
