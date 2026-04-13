<script lang="ts">
    import { getCurrentFile } from "$lib/stores/file.store";
    import Icons from "$lib/utils/icons";
    import { buildRenderList } from "./fileTree";
    import type { FileItem, RenderItem } from "./fileTree";

    export let files: FileItem[] = [];

    // state of open folders
    let openDirs = new Set<string>();

    // reactive flatten list
    $: renderList = buildRenderList(files, openDirs);

    function toggle(item: RenderItem) {
        if (!item.isDir) return;
        if (openDirs.has(item.id)) openDirs.delete(item.id);
        else openDirs.add(item.id);
        openDirs = openDirs; // trigger reactivity
    }

    let selected: string | null = null;
    const onFileClick = (item: RenderItem) => {
        selected = item.id;
        if (item.file) {
            getCurrentFile(item.file?.id, item.file.title);
        }
    };
</script>

{#each renderList as item (item.id)}
    <button
        role={item.isDir ? "treeitem" : "treeitem"}
        aria-expanded={item.isDir ? item.isOpen : undefined}
        style="--depth: {item.depth}"
        class="tree-item"
        class:dir={item.isDir}
        class:active={selected === item.id}
        on:click={() => (item.isDir ? toggle(item) : onFileClick(item))}
        on:keydown={(e) => e.key === "Enter" && toggle(item)}
        tabindex="0"
    >
        <span class="indent"></span>

        {#if item.isDir}
            <span class="chevron" class:open={item.isOpen}>›</span>
        {/if}
        <span class="name">{item.file?.title ?? item.name}</span>
    </button>
{/each}

<style>
    .chevron {
        display: inline-block;
        width: 12px;
        font-size: 12px;
        transition: transform 0.15s;
        transform: rotate(0deg);
    }
    .chevron.open {
        transform: rotate(90deg);
    }

    .name {
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
