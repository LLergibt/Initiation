import {
  HighlightStyle,
  defaultHighlightStyle,
  syntaxHighlighting,
} from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { Prec } from "@codemirror/state";

// Custom highlight styles for Markdown tokens
const customHighlight = HighlightStyle.define([
  {
    tag: tags.heading1,
    fontSize: "1.6rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },

  {
    tag: tags.heading2,
    fontSize: "1.4rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },
  {
    tag: tags.heading3,
    fontSize: "1.2rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },
  {
    tag: tags.heading4,
    fontSize: "1.1rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },
  {
    tag: tags.heading5,
    fontSize: "1.0rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },
  {
    tag: tags.heading6,
    fontSize: "0.9rem",
    fontWeight: "bold",
    fontFamily: "$font-family",
    textDecoration: "none !important",
  },
]);
const highlightExtension = Prec.highest(syntaxHighlighting(customHighlight));
export default highlightExtension;
