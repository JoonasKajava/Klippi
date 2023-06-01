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

<div data-tauri-drag-region class="navbar" id="titlebar">
    <div data-tauri-drag-region class="navbar-start">
        {#await appName then name}
            <span class="text-xl">{name}</span>
        {/await}
    </div>
    <div data-tauri-drag-region class="navbar-center">
        <div class="btn-group btn-group-horizontal">
            <TitleBarLink disabled href="/settings">Settings</TitleBarLink>            
            <TitleBarLink href="/">Video Selector</TitleBarLink>
            <TitleBarLink disabled={$selected_video.length == 0} href="/video-editor">Editor</TitleBarLink>
        </div>
    </div>
    <div data-tauri-drag-region class="navbar-end">
        <ul class="menu menu-horizontal">
            <li>
                <button on:click={appWindow.minimize} class="material-icons">minimize</button>
            </li>
            <li>
                <button on:click={onMaximize} class="material-icons">fullscreen</button>
            </li>
            <li>
                <button on:click={appWindow.close} class="material-icons hover:bg-red-600 hover:text-white">close</button>
            </li>
        </ul>
    </div>
</div>