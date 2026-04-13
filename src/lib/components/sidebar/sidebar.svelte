<script lang="ts">
    import "./styles.scss";
    import Icons from "$lib/utils/icons";
    import TreeCom from "$lib/components/tree/TreeCom.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Pane, type PaneAPI } from "paneforge";
    import { sidebarStore, handleResize, preventGrow } from "./sidebarStore";
    import { createNote, currentDirectory } from "$lib/stores/file.store";
    import Modal from "./modal.svelte";
    import type { FileItem } from "../tree/fileTree";
    import { onMount } from "svelte";

    let { containerWidth } = $props();
    let show = $state(false);
    let container: HTMLDivElement | null = $state(null);

    let files = $state<FileItem[]>();
    const getNodes = async () => {
        files = await invoke("list_nodes");
    };
    onMount(() => {
        getNodes();
    });

    const clickHandler = (event: any) => {
        event.preventDefault();

        if (container?.contains(event.target) == false) {
            show = false;
        }
    };

    // svelte-ignore non_reactive_update
    let paneOne: PaneAPI;
    let isHidden = $state(false);
    $effect(() => {
        sidebarStore.set(paneOne);
        // preventGrow is preventing sidebar to adapt when resizing window
        // its probably not a best way to do that cause its basically an overwrite of default behavior of library and it
        // causes some visual bugs  when resizing but it works
        preventGrow(containerWidth);
    });

    // Some example of data
    const sidebarItems = [
        {
            name: "Заметки о персонажах",
            type: "folder",
            collapsed: true,
            active: false,
        },
        { name: "Initiation Script", type: "file", active: true },
    ];
    const onCreateNote = async (relPath: string) => {
        const file = await createNote(relPath);
        console.log(file);
        files?.push(file);
    };
</script>

<svelte:window on:click={clickHandler} />
<Pane
    onResize={(sizeInPercent) => {
        handleResize(sizeInPercent, containerWidth);
    }}
    collapsible={true}
    collapsedSize={0}
    minSize={20}
    bind:this={paneOne}
    onCollapse={() => {
        isHidden = true;
    }}
    onExpand={() => {
        isHidden = false;
    }}
    style={isHidden
        ? `width: 3rem !important; flex: none !important;`
        : `min-width: 14rem;`}
>
    {#if !isHidden}
        <aside class="sidebar">
            <div class="sidebar-header">
                <button
                    class="icon-btn"
                    onclick={() => {
                        onCreateNote("untit");
                    }}
                >
                    {@html Icons.note}
                </button>
                <button class="icon-btn last-elem" onclick={paneOne.collapse}>
                    {@html Icons.sidebar}
                </button>
            </div>

            <div class="file-tree">
                <TreeCom {files} />
            </div>

            <div bind:this={container} class="dropdown-container">
                {#if show}
                    <Modal />
                {/if}
                <button
                    class="dropdown-toggle"
                    onclick={() => {
                        show = !show;
                    }}
                >
                    <span>
                        {$currentDirectory.name}
                    </span>
                    {@html Icons.select}
                </button>
            </div>
        </aside>
    {:else}
        <aside class="hide-sidebar">
            <div class="sidebar-header">
                <button onclick={paneOne.expand} class="icon-btn">
                    {@html Icons.sidebar}
                </button>
            </div>
        </aside>
    {/if}
</Pane>
