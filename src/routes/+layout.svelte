<script lang="ts">
    import { fly } from "svelte/transition";
    import TitleBar from "./TitleBar.svelte";
    import Update from "$lib/components/Update.svelte";
    import { checkUpdate, type UpdateManifest } from "@tauri-apps/api/updater";

    export let data;

    let should_update = false;
    let update_manifest: UpdateManifest;

    (async () => {
        const { shouldUpdate, manifest } = await checkUpdate();
        should_update = shouldUpdate;
        update_manifest = manifest;
    })();
</script>

<TitleBar />

{#key data.url}
    <div
        in:fly={{ delay: 200, duration: 200, y: 100 }}
        out:fly={{ duration: 200, y: -100 }}
    >
        <slot />
    </div>
{/key}

{#if should_update}
    <Update open={should_update} manifest={update_manifest} />
{/if}
