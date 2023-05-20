<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { clip_end, clip_start } from "../ClipOptions/ClipOptionsStore";
    export let value: number;

    let slider;

    $: console.dir(slider);

    export let seconds;
    const dispatch = createEventDispatcher();
</script>

<div class="p-3">
    <input
    bind:this={slider}
        on:input={(e) => dispatch("input", e)}
        type="range"
        min="0"
        step="0.1"
        max={seconds}
        bind:value
        class="range range-primary"
    />
    <button on:click={() => clip_start.set(value)} class="btn">Set Start</button>
    <button on:click={() => clip_end.set(value)} class="btn">Set End</button>
</div>
