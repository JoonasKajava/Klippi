<script lang="ts">
    import { onMount } from "svelte";


    import { KonvaTimeline } from "./KonvaTimeline";

    export let video_current_time: number;
    export let onUpdate: (value: number) => void;
    export let seconds;

    let width: number;
    let height: number;

    let timeline: KonvaTimeline;

    $: if (timeline) {
        timeline.update_current_time(video_current_time);
    }

    $: if (timeline) {
        timeline.update_stage_dimensions(width, height);
    }

    onMount(() => {
        timeline = new KonvaTimeline(width, height, seconds, onUpdate);
        timeline.create_stage();
        timeline.create_timeline_group();
        timeline.create_current_time_marker();
        for (let i = 0; i < seconds; i++) {
            timeline.create_time_marker(i);
        }
    });
</script>

<div class="h-16" bind:clientWidth={width} bind:clientHeight={height}>
    <div id="stage" on:contextmenu={(e) => e.preventDefault()} />
</div>
