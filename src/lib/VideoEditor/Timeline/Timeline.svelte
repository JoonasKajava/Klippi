<script lang="ts">
    import { onMount } from "svelte";
    import type { Event, UnlistenFn } from "@tauri-apps/api/event";
    import { KonvaTimeline } from "./KonvaTimeline";
    import { invoke } from "@tauri-apps/api/tauri";
    import { selected_video } from "../VideoEditorStore";
    import { thumbnail_processes } from "./TimelineStore";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Progress } from "src/models/Progress";
    import { duration } from "../ClipOptions/ClipOptionsStore";
    import type { TimelineThumbnailsResult } from "src/models/TimelineThumbnailsResult";

    export let video_current_time: number;
    export let onUpdate: (value: number) => void;
    export let seconds: number;

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
        if(!seconds) return;
        timeline = new KonvaTimeline(width, height, seconds, onUpdate);
        timeline.create_stage();
        timeline.create_timeline_group();
        timeline.create_current_time_marker();
        for (let i = 0; i < seconds; i++) {
            timeline.create_time_marker(i);
        }

        // Disable thumbnails for now
        return;
        if (!$thumbnail_processes.includes($selected_video)) {
            thumbnail_processes.add($selected_video);
            invoke<TimelineThumbnailsResult>("get_timeline_thumbnails", {
                of: $selected_video,
                duration: Math.round(seconds),
            }).then((result) => {
                console.log(result);
                if ("Found" in result) {
                    timeline.create_timeline_thumbnails(
                        result.Found,
                        seconds + 2
                    );
                } else {
                    appWindow.listen(
                        "thumbnail_progress",
                        (event: Event<Progress>) => {
                            if (event.payload.status == "End") {
                                thumbnail_processes.remove($selected_video);
                            } else {
                                timeline.create_timeline_thumbnails(
                                    result.Generating,
                                    seconds
                                );
                            }
                        }
                    );
                }
            });
        }
    });
</script>

<div class="h-16" bind:clientWidth={width} bind:clientHeight={height}>
    <div id="stage" on:contextmenu={(e) => e.preventDefault()} />
</div>
