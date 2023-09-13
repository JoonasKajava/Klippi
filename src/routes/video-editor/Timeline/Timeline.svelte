<script lang="ts">
    import { onMount } from 'svelte';
    import { KonvaTimeline } from './KonvaTimeline';
    import { clipStart, duration } from '$lib/stores/ClipOptionsStore';

    export let videoCurrentTime: number;
    export let onUpdate: (value: number) => void;
    export let seconds: number;

    const CENTER_SCALE = 70;

    export function center() {
        timeline.stage_set_scale(
            CENTER_SCALE,
            -videoCurrentTime * CENTER_SCALE * timeline.get_marker_gap() +
            width / 2
        );
    }

    export function updateClipMarker() {
        if ($duration <= 0 || $clipStart === null) return;
        timeline.update_clip_marker($clipStart, $duration);
    }

    let width: number;
    let height: number;

    let timeline: KonvaTimeline;

    $: if (timeline) {
        timeline.update_current_time(videoCurrentTime);
    }

    $: if (timeline) {
        timeline.update_stage_dimensions(width, height);
    }

    onMount(() => {
        if (!seconds) return;
        timeline = new KonvaTimeline(width - 1, height - 1, seconds, onUpdate);

        // Disable thumbnails for now
        /*
                if (!$thumbnailProcesses.includes($selectedVideo)) {
                    thumbnailProcesses.add($selectedVideo);
                    invoke<TimelineThumbnailsResult>('get_timeline_thumbnails', {
                        of: $selectedVideo,
                        duration: Math.round(seconds)
                    }).then((result) => {
                        console.log(result);
                        if ('Found' in result) {
                            timeline.create_timeline_thumbnails(
                                result.Found,
                                seconds + 2
                            );
                        } else {
                            appWindow.listen(
                                'thumbnail_progress',
                                (event: Event<Progress>) => {
                                    if (event.payload.status == 'End') {
                                        thumbnailProcesses.remove($selectedVideo);
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
        */
    });
</script>

<div class="h-16 relative mx-3" bind:offsetWidth={width} bind:offsetHeight={height}>
    <div
            id="stage"
            class="absolute"
            on:contextmenu={(e) => { e.preventDefault(); }}
    />
</div>
