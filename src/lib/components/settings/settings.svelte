<script lang="ts">
    import "./styles.scss";
    let { show = $bindable() } = $props();
    let container: HTMLDivElement | null = $state(null);
    const clickHandler = (event: any) => {
        event.preventDefault();
        console.log("just a click");

        if (container?.contains(event.target) == false) {
            console.log("clicked outside");
            // not ready yet.
            // show = false;
            // console.log(show);
        }
    };
</script>

<svelte:window on:click={clickHandler} />
<div class="modal-overlay show">
    <div bind:this={container} class="modal-window">
        <button
            class="close-modal"
            id="closeModal"
            onclick={() => {
                show = false;
            }}>&times;</button
        >
        <div class="settings__page">
            <div class="header">
                <div class="elems">
                    <h1>Settings</h1>
                    <p>
                        Manage your account settings and set e-mail preferences.
                    </p>
                </div>
                <hr class="divider" />
            </div>

            <div class="body">
                <div class="options">
                    <ul>
                        <li class="menu-item">General</li>
                        <li class="menu-item">Appearance</li>
                        <li class="menu-item">Editor</li>
                        <li class="menu-item">Files</li>
                    </ul>
                </div>
                <div class="content">
                    <h4>Appearance</h4>
                    <p>
                        Turn items on or off to control what's displayed in the
                        app.
                    </p>
                    <hr class="content-divider" />
                </div>
            </div>
        </div>
    </div>
</div>
