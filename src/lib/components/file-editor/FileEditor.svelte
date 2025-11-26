<script lang="ts">
    import "./styles.scss";
    import CodeMirror from "svelte-codemirror-editor";
    import { markdown } from "@codemirror/lang-markdown";
    import { marked } from "marked";

    import { EditorView } from "@codemirror/view";
    // import { HighlightStyle } from "@codemirror/language";
    // import { tags } from "@lezer/highlight";
    const Icons = {
        read: `<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-label="Read mode">
        <path d="M2 12s4-7 10-7 10 7 10 7-4 7-10 7S2 12 2 12Z" stroke="#333" stroke-width="1.5" fill="none"/>
        <circle cx="12" cy="12" r="3.5" stroke="#333" stroke-width="1.5" fill="none"/>
        <circle cx="12" cy="12" r="2" fill="#333"/>
      </svg>`,
        edit: `
        <svg width="24" height="24" viewBox="0 0 24 24"
             xmlns="http://www.w3.org/2000/svg" aria-label="Edit mode">
          <path d="M3.5 16.5L7.5 20.5 20.5 7.5 16.5 3.5 3.5 16.5Z"
                stroke="black" stroke-width="1.5" fill="none"/>
          <path d="M15.8 4.2l4 4"
                stroke="black" stroke-width="1.5" fill="none"/>
          <path d="M7 21l-4 1 1-4"
                stroke="black" stroke-width="1.5" fill="none"/>
        </svg>
      `,
    };

    let isEdit = $state(false);
    let value = $state(`# Initiation Script
it's not a lake it's an **ocean**.`);
    $effect(() => {
        console.log(value);
    });

    // Custom theme (fonts, spacing, gutter hidden)
    const lightTheme = EditorView.theme(
        {
            ".cm-content": {
                fontFamily: "Inter, sans-serif",
                fontSize: "16px",
                lineHeight: "1.6",
                backgroundColor: "white",
                color: "#222",
            },
            ".cm-line": {
                padding: "2px 4px",
            },
            ".cm-gutters": {
                display: "none !important", // hides line numbers gutter
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
    const onChangeMode = () => {
        isEdit = !isEdit;
    };

    // Custom highlight styles for Markdown tokens
    // const customHighlight = HighlightStyle.define([
    //     {
    //         tag: tags.heading1,
    //         fontSize: "1.8em",
    //         fontWeight: "bold",
    //         color: "#ffcc00",
    //     },
    // ]);
</script>

<div class="main-div">
    {#if isEdit}
        <button onclick={onChangeMode} class="btn">
            {@html Icons.read}
        </button>
        <CodeMirror bind:value lang={markdown()} extensions={[lightTheme]}
        ></CodeMirror>
    {:else}
        <button onclick={onChangeMode} class="btn">
            {@html Icons.edit}
        </button>
        {@html marked(value)}
    {/if}
</div>

<style>
    /* Optional: tweak editor container */
    .main-div {
        padding: 3rem;
    }
    .btn {
        background-color: white;
        box-shadow: none;
        border: none;
    }
</style>
