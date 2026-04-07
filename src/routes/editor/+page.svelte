<script lang="ts">
    import Sidebar from "$lib/components/sidebar/sidebar.svelte";
    import { PaneGroup, Pane, PaneResizer } from "paneforge";
    import FileEditor from "$lib/components/file-editor/FileEditor.svelte";
    import { invoke } from "@tauri-apps/api/core";
    const getNodes = async () => {
        // let j = await invoke("create_note", {
        //     relPath: "initiation/",
        //     title: "initiation",
        //     text: "all work and no play makes jack a dull boy",
        // });

        // console.log(j);
        let g = await invoke("list_nodes");
        console.log(g);
    };
    getNodes();

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
    :global(.resizer) {
        width: 1.1px;
        /*background-color: #e4e4e7;*/
        background-color: #e4e4e7;
        cursor: col-resize;
        transition: background-color 0.2s;
    }

    :global(.resizer:hover),
    :global(.resizer[data-active]) {
        /*background-color: #e4e4e9;*/
    }
</style>
