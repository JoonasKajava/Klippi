<script type="ts">
    import { fly } from "svelte/transition";
    import TitleBar from "./TitleBar.svelte";
    import Update from "$lib/components/Update.svelte";
    import { checkUpdate } from "@tauri-apps/api/updater";

    export let data;

    let should_update = false;
    let update_manifest;

    (async () => {
        const { shouldUpdate, manifest } = await checkUpdate();
        //should_update = shouldUpdate;
        update_manifest = manifest;
    })();

    setTimeout(() => {
        should_update = true;
    }, 4000);
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
