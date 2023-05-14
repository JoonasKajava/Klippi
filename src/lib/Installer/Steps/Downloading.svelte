<script lang="ts">
    import type { Event } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";
    import prettyBytes from "pretty-bytes";

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

    appWindow.listen("download_progress", (event: Event<Progress>) => {
        progress = event.payload;
    });
</script>

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
