<script lang="ts">
    import "./styles.scss";
    import Icons from "$lib/utils/icons";
    // Заглушки пока Vladimir GURSKY не сделал апи с файловой системой
    let isHidden = $state(false);
    const hideSidebar = () => {
        isHidden = !isHidden;
    };
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

{#if isHidden}
    <aside class="hide-sidebar">
        <div class="sidebar-header">
            <button onclick={hideSidebar} class="icon-btn last-elem">
                {@html Icons.sidebar}
            </button>
        </div>
        <div class="sidebar-footer">
            <div class="user-profile">
                <div class="avatar">A</div>
            </div>
        </div>
    </aside>
{:else}
    <aside class="sidebar">
        <div class="sidebar-header">
            <div class="icon-btn">{@html Icons.folder}</div>
            <div class="icon-btn">{@html Icons.search}</div>
            <button onclick={hideSidebar} class="icon-btn last-elem">
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
{/if}
