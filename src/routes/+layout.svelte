<script lang="ts">
    import { fly } from 'svelte/transition';
    import TitleBar from './TitleBar.svelte';
    //import Update from '$lib/components/Update.svelte';
    //import { checkUpdate, type UpdateManifest } from '@tauri-apps/plugin-updater';

    export let data;

    let updateAvailable = false;
    let updateManifest: UpdateManifest | undefined;

    void (async () => {
        const { shouldUpdate, manifest } = await checkUpdate();
        updateAvailable = shouldUpdate;
        updateManifest = manifest;
    })();
</script>

<TitleBar/>

{#key data.url}
    <div
            in:fly={{ delay: 200, duration: 200, y: 100 }}
            out:fly={{ duration: 200, y: -100 }}
    >
        <slot/>
    </div>
{/key}

<!-- {#if updateAvailable} -->
<!--     <Update open={updateAvailable} manifest={updateManifest}/> -->
<!-- {/if} -->
