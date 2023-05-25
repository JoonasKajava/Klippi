<script lang="ts">
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Progress } from "src/models/Progress";
    import { onDestroy } from "svelte";
    import { duration } from "../VideoEditor/ClipOptions/ClipOptionsStore";

    let unlisten: UnlistenFn;

    let progress: Progress;
    let progress_value: number;

    $: if(progress) {
        progress_value = progress && isFinite(progress.out_time) && progress.out_time > 0 ? Math.min(progress.out_time/$duration*100, 100) : 0;
    }

    appWindow
        .listen("ffmpeg_progress", (event: Event<Progress>) => {
            console.log(event);
            progress = event.payload;
        })
        .then((handle) => (unlisten = handle));

    onDestroy(() => {
        unlisten();
    });
</script>

<div class="flex flex-col items-center">
    <h1 class="text-4xl font-bold text-center mt-16 mb-8">Processing Clip!</h1>
    {#if progress}
        <progress class="progress progress-info w-56" value={progress_value} max="100" />

        {progress_value.toFixed(1)}%

        <div class="stats shadow">
            <div class="stat">
                <div class="stat-title">Processing Speed</div>
                <div class="stat-value font-mono">{progress.fps?.toFixed(1)} fps</div>
            </div>

            <div class="stat">
                <div class="stat-title">Processed</div>
                <div class="stat-value font-mono">{progress.out_time?.toFixed(1)}sec</div>
            </div>

            <div class="stat">
                <div class="stat-title">Processing left</div>
                <div class="stat-value font-mono">{(Math.max($duration - progress.out_time, 0)).toFixed(1)}sec</div>
            </div>
        </div>
    {/if}
</div>
