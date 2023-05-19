<script lang="ts">
    import {getName} from '@tauri-apps/api/app';
    import {appWindow} from '@tauri-apps/api/window';
    import { selected_video } from './VideoEditor/VideoEditorStore';
    import VideoSelector from './VideoSelector.svelte';
    import VideoEditor from './VideoEditor/VideoEditor.svelte';
    import type { ComponentType } from 'svelte';
    import { current_page } from './shared/AppStore';

    const appName = getName();

    let video;

    async function onMaximize() {
        await appWindow.isMaximized() ? appWindow.unmaximize() : appWindow.maximize();
    }

    function go_to(page: ComponentType) {
        if($current_page == page) return;
        $current_page = page;
    }

    selected_video.subscribe((v) => video = v);
</script>

<div data-tauri-drag-region class="navbar">
    <div data-tauri-drag-region class="navbar-start">
        {#await appName then name}
            <span class="text-xl">{name}</span>
        {/await}
    </div>
    <div data-tauri-drag-region class="navbar-center">
        <div class="btn-group btn-group-horizontal">
            <button class="btn">Settings</button>
            <button class="btn" class:btn-active={$current_page == VideoSelector} on:click={() => go_to(VideoSelector)}>Video Selector</button>
            <button class="btn" class:btn-active={$current_page == VideoEditor} disabled={!video} on:click={() => go_to(VideoEditor)}>Editor</button>
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