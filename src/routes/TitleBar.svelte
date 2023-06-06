<script lang="ts">
    import {getName} from '@tauri-apps/api/app';
    import {appWindow} from '@tauri-apps/api/window';
    import TitleBarLink from './TitleBarLink.svelte';
    import { selected_video } from '$lib/stores/VideoEditorStore';

    const appName = getName();

    async function onMaximize() {
        await appWindow.isMaximized() ? appWindow.unmaximize() : appWindow.maximize();
    }

</script>

<div data-tauri-drag-region class="navbar pt-0 pr-0 items-start" id="titlebar">
    <div data-tauri-drag-region class="navbar-start">
        {#await appName then name}
            <span class="text-xl">{name}</span>
        {/await}
    </div>
    <div data-tauri-drag-region class="navbar-center">
        <div class="join join-horizontal">
            <TitleBarLink disabled={true} href="/settings"><i class="fa-solid fa-gear"></i> Settings</TitleBarLink>            
            <TitleBarLink href="/"><i class="fa-regular fa-file-video"></i> Video Selector</TitleBarLink>
            <TitleBarLink disabled={$selected_video.length == 0} href="/video-editor"><i class="fa-solid fa-clapperboard"></i> Editor</TitleBarLink>
        </div>
    </div>
    <div data-tauri-drag-region class="navbar-end">
        <ul class="menu menu-horizontal pt-0 pr-0">
            <li>
                <button class="rounded-none" on:click={appWindow.minimize}><i class="fas fa-window-minimize"></i></button>
            </li>
            <li>
                <button class="rounded-none" on:click={onMaximize}><i class="far fa-window-maximize"></i></button>
            </li>
            <li>
                <button on:click={appWindow.close} class="hover:bg-red-600 hover:text-white rounded-none"><i class="fa-solid fa-xmark"></i></button>
            </li>
        </ul>
    </div>
</div>