<script type="ts">
    import { fly } from "svelte/transition";
    import TitleBar from "./TitleBar.svelte";

    export let data;
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
