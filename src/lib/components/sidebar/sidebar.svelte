<script lang="ts">
    import "./styles.scss";
    import Icons from "$lib/utils/icons";
    import { Pane, type PaneAPI } from "paneforge";
    import { sidebarStore, handleResize, preventGrow } from "./sidebarStore";
    let { containerWidth } = $props();

    // svelte-ignore non_reactive_update
    let paneOne: PaneAPI;
    let isHidden = $state(false);
    $effect(() => {
        sidebarStore.set(paneOne);
        // preventGrow is preventing sidebar to adapt when resizing window
        // its probably not a best way to do that cause its overwrite default behavior of library and
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
</script>

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
                <div class="icon-btn">{@html Icons.folder}</div>
                <div class="icon-btn">{@html Icons.search}</div>
                <button class="icon-btn last-elem" onclick={paneOne.collapse}>
                    {@html Icons.sidebar}
                </button>
            </div>

            <div class="file-tree">
                {#each sidebarItems as item}
                    <div class="tree-item {item.active ? 'active' : ''}">
                        {#if item.type === "folder"}
                            <span class="arrow">
                                {@html item.collapsed
                                    ? Icons.arrowRight
                                    : Icons.arrowDown}
                            </span>
                        {:else}
                            <span class="spacer"></span>
                        {/if}
                        <span class="item-name">{item.name}</span>
                    </div>
                {/each}
            </div>

            <div class="sidebar-footer">
                <div class="user-profile">
                    <div class="avatar">A</div>
                    <span>Alan Wake 3</span>
                </div>
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
