<script lang="ts">
    import { onMount } from "svelte";
    import type { Event } from "@tauri-apps/api/event";
    import { KonvaTimeline } from "./KonvaTimeline";
    import { invoke } from "@tauri-apps/api/tauri";
    import { selected_video } from "../VideoEditorStore";
    import { thumbnail_processes } from "./TimelineStore";
    import { appWindow } from "@tauri-apps/api/window";
    import type { Progress } from "src/models/Progress";
    import type { TimelineThumbnailsResult } from "src/models/TimelineThumbnailsResult";
    import {clip_start, duration } from "../ClipOptions/ClipOptionsStore";

    export let video_current_time: number;
    export let onUpdate: (value: number) => void;
    export let seconds: number;

    const CENTER_SCALE = 70;

    export function center() {
        timeline.stage_set_scale(
            CENTER_SCALE,
            -video_current_time * CENTER_SCALE * timeline.get_marker_gap() +
                width / 2
        );
    }


    export function update_clip_marker() {
        if($duration <= 0) return;
        timeline.update_clip_marker($clip_start, $duration);
    }

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
        if (!seconds) return;
        timeline = new KonvaTimeline(width - 1, height - 1, seconds, onUpdate);
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

<div class="h-16 relative mx-3" bind:offsetWidth={width} bind:offsetHeight={height}>
    <div
        id="stage"
        class="absolute"
        on:contextmenu={(e) => e.preventDefault()}
    />
</div>
