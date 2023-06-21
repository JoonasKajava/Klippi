<script lang="ts">
    import { error } from "@sveltejs/kit";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { appWindow } from "@tauri-apps/api/window";
    import prettyBytes from "pretty-bytes";
    import Step from "./Step.svelte";
    import { onDestroy, onMount } from "svelte";
    import type { DownloadProgress } from "$lib/models/DownloadProgress";

    let progress: DownloadProgress = {
        progress: 0,
        total_size: 0n,
        downloaded: 0n,
        speed: 0,
    };

    function install_dependencies() {
        invoke("install_dependencies").catch((e: string) => {
            throw error(500, {
                message: e,
                title: "Unable to install dependencies",
            });
        });
    }

    let unlisten: UnlistenFn;

    appWindow
        .listen("download_progress", (event: Event<DownloadProgress>) => {
            progress = event.payload;
        })
        .then((handle) => {
            unlisten = handle;
        });

    onDestroy(() => {
        unlisten();
    });

    onMount(() => {
        install_dependencies();
    });

    export let transitions;
</script>

<Step {transitions}>
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
            <div class="stat-value">
                {prettyBytes(Number(progress.total_size))}
            </div>
        </div>
        <div class="stat">
            <div class="stat-title">Downloaded</div>
            <div class="stat-value">
                {prettyBytes(Number(progress.downloaded))}
            </div>
        </div>
    </div>
</Step>
