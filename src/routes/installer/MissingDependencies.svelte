<script lang="ts">
    import { getName } from "@tauri-apps/api/app";
    import { missing_dependencies } from "$lib/stores/InstallerStore";
    import { appWindow } from "@tauri-apps/api/window";
    import DependencyDisplay from "./DependencyDisplay.svelte";
    let appName: String;
    getName().then((name) => (appName = name));

    export let onInstallClick: () => void;

    let modal: HTMLElement;
</script>

<div data-tauri-drag-region bind:this={modal} class="modal">
    <div class="modal-box">
        <button
            on:click={() => modal.classList.remove("modal-open")}
            class="btn btn-sm btn-circle absolute right-2 top-2"
        >
            <i class="fa-solid fa-xmark" />
        </button>
        <h3 class="font-bold text-lg">
            Come back when you are done installing!
        </h3>
        <p>
            {appName} needs to be restarted after you have installed all the neccessary
            programs.
        </p>
        <p>Here's again all the programs you need to install:</p>
        <DependencyDisplay dependencies={$missing_dependencies} />
        <div class="modal-action">
            <button on:click={appWindow.close} class="btn">
                Close {appName}
            </button>
        </div>
    </div>
</div>

<div class="hero">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-3xl font-bold">
                Seems like you are missing something.
            </h1>
            <p class="py-5">
                In order to function correctly {appName} needs following programs:
            </p>
            <DependencyDisplay dependencies={$missing_dependencies} />
            <div class="mt-4 flex">
                <button
                    on:click={() => modal.classList.add("modal-open")}
                    class="btn btn-outline btn-error"
                >
                    I will install these my self
                </button>
                <div class="divider divider-horizontal">OR</div>
                <button
                    on:click={onInstallClick}
                    class="btn btn-outline btn-success"
                >
                    Install these for me
                    <i class="fas fa-chevron-right"></i>
                </button>
            </div>
        </div>
    </div>
</div>
