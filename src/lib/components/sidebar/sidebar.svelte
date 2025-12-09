<script lang="ts">
    import "./styles.scss";
    import Icons from "$lib/utils/icons";
    import { PaneGroup, Pane, PaneResizer } from "paneforge";

    let isHidden = $state(false);
    // svelte-ignore non_reactive_update
    let paneOne: ReturnType<typeof Pane>;

    const hideSidebar = () => {
        if (isHidden) {
            paneOne.expand;
        } else {
            paneOne.collapse;
        }
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
</script>

<Pane
    collapsible={true}
    collapsedSize={0}
    bind:this={paneOne}
    onCollapse={() => (isHidden = true)}
    onExpand={() => (isHidden = false)}
    minSize={20}
    defaultSize={21}
    style={isHidden ? `width: 3rem !important; flex: none !important;` : ""}
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
