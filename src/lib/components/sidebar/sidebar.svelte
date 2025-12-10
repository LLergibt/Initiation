<script lang="ts">
    import "./styles.scss";
    import Icons from "$lib/utils/icons";
    import { PaneGroup, Pane, PaneResizer } from "paneforge";
    let { containerWidth } = $props();

    let isHidden = $state(false);
    // svelte-ignore non_reactive_update
    let paneOne: ReturnType<typeof Pane>;
    let sidebarWidthPx = 224;

    const hideSidebar = () => {
        isHidden = !isHidden;
    };
    // Заглушки пока Vladimir GURSKY не сделал апи с файловой системой
    const sidebarItems = [
        {
            name: "Заметки о персонажах",
            type: "folder",
            collapsed: true,
            active: false,
        },
        { name: "Initiation Script", type: "file", active: true },
    ];
    const handleResize = async (sizeInPercent: number) => {
        if (containerWidth) {
            sidebarWidthPx = (sizeInPercent / 100) * containerWidth;
        }
    };
    $effect(() => {
        if (paneOne && containerWidth && sidebarWidthPx) {
            const newPercent = (sidebarWidthPx / containerWidth) * 100;
            paneOne.resize(newPercent);
        }
    });
</script>

<Pane
    onResize={handleResize}
    collapsible={true}
    collapsedSize={0}
    bind:this={paneOne}
    onCollapse={() => (isHidden = true)}
    onExpand={() => (isHidden = false)}
    defaultSize={14}
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
                <!-- <button onclick={hideSidebar} class="icon-btn last-elem">
                </button> -->
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
