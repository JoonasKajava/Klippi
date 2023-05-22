<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { clip_end, clip_start } from "../ClipOptions/ClipOptionsStore";
    import { Stage, Layer, Rect } from "svelte-konva";
    export let video_current_time: number;

    let slider;

    let width: number;
    let height: number;

    $: console.dir(slider);

    export let seconds;
    const dispatch = createEventDispatcher();
</script>

<div class="h-16" bind:clientWidth={width} bind:clientHeight={height}>
    <Stage config={{width,height }}>
        <Layer>
            <Rect config={{ x: 0, y: 0, width, height: 50, fill: "red" }} />
        </Layer>
    </Stage>
</div>
<div class="p-3">
    <input
        bind:this={slider}
        on:input={(e) => dispatch("input", e)}
        type="range"
        min="0"
        step="0.1"
        max={seconds}
        bind:value={video_current_time}
        class="range range-primary"
    />
    <button on:click={() => clip_start.set(video_current_time)} class="btn">Set Start</button
    >
    <button on:click={() => clip_end.set(video_current_time)} class="btn">Set End</button>
</div>
