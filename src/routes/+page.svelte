<script lang="ts">
    import switchWindow from "$lib/utils/windows";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { onMount } from "svelte";

    const tracks = [
        { title: "Departure" },
        { title: "Initiation" },
        { title: "Return" },
        { title: "Prologue" },
    ];

    let currentIndex = $state(0);

    function selectTrack(index: number) {
        currentIndex = index;
    }

    function nextTrack() {
        if (currentIndex < tracks.length - 1) {
            currentIndex++;
        }
    }

    function prevTrack() {
        if (currentIndex > 0) {
            currentIndex--;
        }
    }

    function handleKey(e: KeyboardEvent) {
        if (["ArrowDown", "j", "ArrowRight", "l"].includes(e.key)) {
            e.preventDefault();
            nextTrack();
        } else if (["ArrowUp", "k", "ArrowLeft", "h"].includes(e.key)) {
            e.preventDefault();
            prevTrack();
        }
    }

    onMount(() => {
        window.addEventListener("keydown", handleKey);

        return () => {
            window.removeEventListener("keydown", handleKey);
        };
    });
    async function selectWorkspace() {
        console.log("here");
        const dir = await open({ directory: true });

        if (dir) {
            console.log(dir);
            await invoke("open_fs", { root: dir });
            await switchWindow();
        }
    }
    // console.log(file);

    let containerWidth = $state();
</script>

<div class="main-body" style="height: 100vh;" bind:clientWidth={containerWidth}>
    <div class="player-window">
        <div class="main-content">
            <div class="left-panel">
                <div class="palette-container-form">
                    <div class="color-form-top">
                        <div class="form-header">
                            <span class="inspiration-text">
                                未知への招待状
                            </span>
                            <h1 class="color-title">Initiation</h1>
                            <div class="button-info">
                                <button onclick={selectWorkspace}
                                    >Open project</button
                                >
                                <p>Create new project</p>
                            </div>
                        </div>
                        <div class="label-info">
                            <p>HTV-TT5SAE532(B) E182155 T</p>
                            <p>C P/N: H2T320854S7 5400 RPM</p>
                            <p>HDD: Z5K2360320</p>
                            <p>320GB SATA 3.0GB/S</p>
                        </div>
                    </div>
                    <div class="color-form-split">
                        <div class="form-block slate-gray-background">
                            <p class="form-color-name pink-text">
                                帰路
                                <span>Return</span>
                            </p>
                            <p class="form-color-name pink-text">Return</p>
                            <div class="form-tech-details">
                                <p>HDD: Z5K2360320 320GB SATA</p>
                                <p>MLC: DA475584 G</p>
                            </div>
                        </div>
                        <div class="form-block camouflage-sand-background">
                            <p class="form-color-name dark-text">
                                旅立ち <span>Departure</span>
                            </p>
                            <p class="form-color-name dark-text">Departure</p>
                            <div class="form-tech-details dark-text">
                                <p>MLC: DA475584 G</p>
                                <p>MADE IN CONTRAST WITH LOVE</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="right-panel">
                <ul class="playlist" id="playlist">
                    {#each tracks as track, index}
                        <button
                            class="track {index === currentIndex
                                ? 'active'
                                : ''}"
                            onclick={() => selectTrack(index)}
                        >
                            <div
                                style="display: flex; flex-direction: column; gap: 4px;"
                            >
                                <span>Alan Wake - {track.title}</span>
                                <span style="font-size: 0.85rem; opacity: 0.6;">
                                    /Users/georgijnazarov/Documents
                                </span>
                            </div>

                            <svg class="settings-icon" viewBox="0 0 24 24">
                                <path
                                    d="M6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"
                                />
                            </svg>
                        </button>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
</div>
