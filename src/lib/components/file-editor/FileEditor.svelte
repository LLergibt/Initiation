<script lang="ts">
    import CodeMirror from "svelte-codemirror-editor";
    import { EditorView } from "@codemirror/view";
    import { currentFile } from "$lib/stores/file.store";
    // import { currentFile } from "$lib/stores";

    import { markdown } from "@codemirror/lang-markdown";
    import {
        defaultHighlightStyle,
        syntaxHighlighting,
    } from "@codemirror/language";

    import { marked } from "marked";

    import "./styles.scss";
    import lightTheme from "./lightTheme";
    import highlightExtension from "./highlightExtension";

    import Icons from "$lib/utils/icons";
    const ModesIcons = {
        edit: `
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24">
        <path fill="#000000" d="m19.71 8.04l-2.34 2.33l-3.75-3.75l2.34-2.33c.39-.39 1.04-.39 1.41 0l2.34 2.34c.39.37.39 1.02 0
        1.41M3 17.25L13.06 7.18l3.75 3.75L6.75 21H3v-3.75M16.62 5.04l-1.54 1.54l2.34 2.34l1.54-1.54l-2.34-2.34M15.36 11L13
        8.64l-9 9.02V20h2.34l9.02-9Z"/></svg>
        `,
        read: `
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24"><path fill="#000000" d="M12 18.883a10.8 10.8 0 0 1-9.675-5.728a2.6 2.6 0 0 1 0-2.31A10.8 10.8 0 0 1 12 5.117a10.8 10.8 0 0 1 9.675 5.728a2.6 2.6 0 0 1 0 2.31A10.8 10.8 0 0 1 12 18.883Zm0-12.766a9.787 9.787 0 0 0-8.78 5.176a1.586 1.586 0 0 0 0 1.415A9.788 9.788 0 0 0 12 17.883a9.787 9.787 0 0 0 8.78-5.176a1.584 1.584 0 0 0 0-1.414A9.787 9.787 0 0 0 12 6.117Z"/><path fill="#000000" d="M12 16.049A4.049 4.049 0 1 1 16.049 12A4.054 4.054 0 0 1 12 16.049Zm0-7.1A3.049 3.049 0 1 0 15.049 12A3.052 3.052 0 0 0 12 8.951Z"/><circle cx="12" cy="12" r="2.028" fill="#000000"/></svg>

          `,
    };

    let isEdit = $state(false);
    const onChangeMode = () => {
        isEdit = !isEdit;
    };

    const defaultHighlight = syntaxHighlighting(defaultHighlightStyle);
</script>

<div class="content">
    <div class="top-bar">
        <div class="navigation-arrows">
            <span>←</span>
            <span>→</span>
        </div>
        <div class="tab active">
            <span>Initiation Script</span>
            <span class="close-btn">{@html Icons.close}</span>
        </div>
        <div class="new-tab">{@html Icons.plus}</div>
        <button onclick={onChangeMode} class="btn last-elem">
            {#if isEdit}
                {@html ModesIcons.read}
            {:else}
                {@html ModesIcons.edit}
            {/if}
        </button>
    </div>
    <div class="editor-area">
        <div class="editor-width-limiter">
            <div>
                <input
                    class="page-title editable"
                    bind:value={$currentFile.name}
                />
            </div>
            <div class="page-content">
                {#if isEdit}
                    <CodeMirror
                        bind:value={$currentFile.data}
                        lang={markdown()}
                        extensions={[
                            defaultHighlight,
                            lightTheme,
                            highlightExtension,
                        ]}
                        lineWrapping={true}
                    >
                        > ></CodeMirror
                    >
                {:else}
                    <div class="markdown wrap">
                        {@html marked.parse($currentFile.data)}
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>

<style>
    .markdown :global(h1) {
        font-size: 1.6em;
    }
    .markdown :global(h2) {
        font-size: 1.4em;
    }
    .markdown :global(li) {
        margin-left: 1.5rem;
    }
    .markdown :global(li::marker) {
        color: #c2c2c2;
    }
</style>
