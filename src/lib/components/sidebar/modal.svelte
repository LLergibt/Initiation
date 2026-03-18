<script lang="ts">
    import Settings from "$lib/components/settings/settings.svelte";
    import "./styles.scss";
    let show = $state(false);
    const handleSettings = () => {
        show = true;
    };
    $effect(() => {
        console.log(show);
    });
</script>

<ul class="dropdown-menu show">
    <li class="menu-item">
        <button class="menu-btn" onclick={handleSettings}> Dashboard </button>
    </li>
    <li class="menu-item">
        <button class="menu-btn" onclick={handleSettings}> Settings </button>
    </li>
</ul>
{#if show}
    <Settings bind:show />
{/if}
