<script lang="ts">
    import { getName } from '@tauri-apps/api/app';
    import { missingDependencies } from '$lib/stores/InstallerStore';
    import { appWindow } from '@tauri-apps/api/window';
    import DependencyDisplay from './DependencyDisplay.svelte';

    let appName: string;
    getName().then((name) => (appName = name)).catch(console.error);

    export let onInstallClick: () => void;

    let modal: HTMLElement;
</script>

<div data-tauri-drag-region bind:this={modal} class="modal">
    <div class="modal-box">
        <button
                on:click={() => { modal.classList.remove('modal-open'); }}
                class="btn btn-sm btn-circle absolute right-2 top-2"
        >
            <i class="fa-solid fa-xmark"/>
        </button>
        <h3 class="font-bold text-lg">
            Come back when you are done installing!
        </h3>
        <p>
            {appName} needs to be restarted after you have installed all the necessary
            programs.
        </p>
        <p>Here's again all the programs you need to install:</p>
        <DependencyDisplay dependencies={$missingDependencies}/>
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
            <DependencyDisplay dependencies={$missingDependencies}/>
            <div class="mt-4 flex justify-center">
                <button
                        on:click={onInstallClick}
                        class="btn btn-outline btn-success"
                >
                    Install
                    <i class="fas fa-chevron-right"></i>
                </button>
            </div>
        </div>
    </div>
</div>
