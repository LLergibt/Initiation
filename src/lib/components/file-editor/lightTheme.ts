import { EditorView } from "@codemirror/view";

const lightTheme = EditorView.theme(
  {
    ".cm-content": {
      fontFamily:
        '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif',
      fontSize: "1rem",
      lineHeight: "1.6",
      color: "#222",
    },

    ".cm-line": {
      padding: "2px 4px",
    },
    ".ͼ5": {
      color: "#B3B3B3",
    },

    ".cm-activeLine": {
      background: "transparent !important",
    },

    ".cm-selectionBackground, ::selection": {
      background: "#DADADA !important",
    },
    "&.cm-focused .cm-selectionBackground, ::selection": {
      background: "#DADADA !important",
    },

    ".cm-selectionMatch": {
      backgroundColor: "transparent !important",
    },
    ".cm-gutters": {
      display: "none !important", // hides line numbers gutter
      backgroundColor: "transparent",
    },
    ".cm-editor": {
      backgroundColor: "white",
      border: "none",
      boxShadow: "none",
    },
    ".cm-cursor": {
      borderLeftColor: "#000", // black cursor
    },
  },
  { dark: false },
);
export default lightTheme;
