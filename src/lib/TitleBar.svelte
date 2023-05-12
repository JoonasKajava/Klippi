<script lang="ts">
    import {getName} from '@tauri-apps/api/app';
    import {appWindow} from '@tauri-apps/api/window';
    const appName = getName();

    async function onMaximize() {
        await appWindow.isMaximized() ? appWindow.unmaximize() : appWindow.maximize();
    }
</script>

<div data-tauri-drag-region class="navbar">
    <div data-tauri-drag-region class="navbar-start">
        {#await appName then name}
            <span class="text-xl">{name}</span>
        {/await}
    </div>
    <div data-tauri-drag-region class="navbar-center">
        <ul class="menu menu-horizontal">
            <li>
                <button>Settings</button>
            </li>
            <li>
                <button>Video Selector</button>
            </li>
            <li>
                <button>Editor</button>
            </li>
        </ul>
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