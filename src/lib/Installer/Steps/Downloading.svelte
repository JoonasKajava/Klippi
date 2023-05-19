<script lang="ts">
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";
    import prettyBytes from "pretty-bytes";
    import { fly } from "svelte/transition";
    import Step from "./Step.svelte";
    import { onDestroy } from 'svelte';

    interface Progress {
        progress: number;
        total_size: number;
        downloaded: number;
        speed: number;
    }

    export let onNext: () => void;
    let progress: Progress = {
        progress: 0,
        total_size: 0,
        downloaded: 0,
        speed: 0,
    };

    let unlisten : UnlistenFn;

    appWindow.listen("download_progress", (event: Event<Progress>) => {
        progress = event.payload;
    }).then((handle) => {
        unlisten = handle;
    });

    onDestroy(() => {
        unlisten();
    })

    export let transitions;
</script>

<Step transitions={transitions}>
    <div class="mt-10">
        <h1>Downloading FFmpeg</h1>
    </div>
    <progress class="progress w-56" value={progress.progress} max="1" />
    {(progress.progress * 100).toFixed(1)}%
    <div class="stats shadow">
        <div class="stat">
            <div class="stat-title">Download Speed</div>
            <div class="stat-value">{prettyBytes(progress.speed)}</div>
        </div>

        <div class="stat">
            <div class="stat-title">File Size</div>
            <div class="stat-value">{prettyBytes(progress.total_size)}</div>
        </div>
        <div class="stat">
            <div class="stat-title">Downloaded</div>
            <div class="stat-value">{prettyBytes(progress.downloaded)}</div>
        </div>
    </div>
</Step>
