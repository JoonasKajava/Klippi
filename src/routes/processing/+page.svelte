<script lang="ts">
    import type { Progress } from '$lib/models/Progress';
    import { duration } from '$lib/stores/ClipOptionsStore';
    import type { Event, UnlistenFn } from '@tauri-apps/api/event';
    import { appWindow } from '@tauri-apps/api/window';
    import { onDestroy } from 'svelte';

    let unlisten: UnlistenFn;

    let progress: Progress;
    let progressValue: number;

    $: if (progress?.out_time) {
        progressValue = progress && isFinite(progress.out_time) && progress.out_time > 0 ? Math.min(progress.out_time / $duration * 100, 100) : 0;
    }

    appWindow
        .listen('ffmpeg_progress', (event: Event<Progress>) => {
            console.log(event);
            progress = event.payload;
        })
        .then((handle) => (unlisten = handle)).catch(console.error);

    onDestroy(() => {
        unlisten();
    });
</script>

<div class="flex flex-col items-center">
    <h1 class="text-4xl font-bold text-center mt-16 mb-8">Processing Clip!</h1>
    {#if progress}
        <progress class="progress progress-info w-56" value={progressValue} max="100"/>

        {progressValue.toFixed(1)}%

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
                <div class="stat-value font-mono">{(Math.max($duration - (progress?.out_time ?? 0), 0)).toFixed(1)}sec
                </div>
            </div>
        </div>
    {/if}
</div>
