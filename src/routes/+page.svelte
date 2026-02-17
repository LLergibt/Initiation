<script lang="ts">
    import Sidebar from "$lib/components/sidebar/sidebar.svelte";
    import { PaneGroup, Pane, PaneResizer } from "paneforge";
    import FileEditor from "$lib/components/file-editor/FileEditor.svelte";
    let containerWidth = $state();
</script>

<div style="height: 100vh;" bind:clientWidth={containerWidth}>
    <PaneGroup direction="horizontal">
        <Sidebar {containerWidth} />
        <PaneResizer class="resizer" />
        <Pane minSize={20} defaultSize={80}>
            <FileEditor />
        </Pane>
    </PaneGroup>
</div>

<style>
    /* Styling the resizer handle */
    :global(.resizer) {
        width: 1.1px;
        background-color: #e4e4e7;
        cursor: col-resize;
        transition: background-color 0.2s;
    }

    :global(.resizer:hover),
    :global(.resizer[data-active]) {
        background-color: #3b82f6; /* Blue highlight on hover/active */
    }
</style>
