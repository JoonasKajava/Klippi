<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { fly } from "svelte/transition";
    import VolumeController from "./VolumeController.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import Timeline from "../Timeline/Timeline.svelte";
    import {
        clip_end,
        clip_start,
        duration,
        speed,
    } from "../ClipOptions/ClipOptionsStore";

    export let video: string;
    let timeline: Timeline;
    let video_player: HTMLVideoElement;
    let is_playing: boolean;
    let video_fullscreen = false;
    let display_controls = false;
    let loop_clip = false;

    let video_duration: number;
    let current_time: number = 0;
    let volume = 0.5;

    let timeline_height: number;
    let window_height: number;

    const FRAME_RATE = 60;

    function move_one_frame(direction: -1 | 1) {
        video_player.pause();
        video_player.currentTime += direction / FRAME_RATE;
    }

    function toggle_play() {
        video_player.paused ? video_player.play() : video_player.pause();
        is_playing = !video_player.paused;
    }

    function timeupdate() {
        if (loop_clip && $duration > 0) {
            if (
                video_player.currentTime > $clip_end ||
                video_player.currentTime < $clip_start - 1
            ) {
                video_player.currentTime = $clip_start;
            }
        }

        current_time = video_player.currentTime;
    }

    function pretty_seconds(sec: number) {
        return `${Math.floor(sec / 60)}:${(sec % 60)
            .toFixed(0)
            .padStart(2, "0")}`;
    }

    async function toggle_fullscreen() {
        appWindow.isFullscreen().then((value) => {
            appWindow.setFullscreen(!value);
            video_fullscreen = !value;
        });
    }

    $: if (video_player) {
        video_player.volume = volume;
        video_player.playbackRate = $speed;
    }

    function exit_fullscreen() {
        appWindow.isFullscreen().then((value) => {
            if (value) toggle_fullscreen();
        });
    }

    function set_clip_start() {
        $clip_start = video_player.currentTime;
        timeline.update_clip_marker();
    }

    function set_clip_end() {
        $clip_end = video_player.currentTime;
        timeline.update_clip_marker();
    }

    function keyboard_navigation(
        e: KeyboardEvent & { currentTarget: EventTarget & Window, target: HTMLElement }
    ) {
        console.log(e);
        if(e.target.nodeName.toLowerCase() === 'input') return;
        switch (e.code) {
            case "ArrowLeft":
                move_one_frame(-1);
                break;
            case "ArrowRight":
                move_one_frame(1);
                break;
            case "ArrowUp":
                set_clip_start();
                break;
            case "ArrowDown":
                set_clip_end();   
                break;
            case "Space":
                timeline.center();
                break;
        }
    }

    function update_video_max_Height() {
        video_player.style.maxHeight = `${
            window_height -
            timeline_height -
            document.getElementById("titlebar").clientHeight -
            20
        }px`;
    }
</script>

<svelte:window
    on:resize={update_video_max_Height}
    on:keyup={exit_fullscreen}
    on:keydown={keyboard_navigation}
/>

<div
    class:fullscreen={video_fullscreen}
    on:mouseenter={() => (display_controls = true)}
    on:mouseleave={() => (display_controls = false)}
    class="relative inline-flex flex-col video-player transition-all z-50"
>
    <!-- svelte-ignore a11y-media-has-caption -->
    <video
        on:click={toggle_play}
        on:timeupdate={timeupdate}
        on:durationchange={(d) => (video_duration = video_player.duration)}
        bind:this={video_player}
    >
        <source src={convertFileSrc(video, "stream")} />
    </video>
    {#if display_controls}
        <div
            transition:fly={{ y: 5, duration: 200 }}
            class="absolute inset-0 controls transition-all flex flex-col"
        >
            <div
                class="grow"
                on:click={toggle_play}
                on:keypress={toggle_play}
            />

            <div
                class=" self-end p-2 w-full bg-gradient-to-t from-black from-20% to-transparent text-white"
            >
                <div class="flex justify-between">
                    <div class="flex items-center gap-4">
                        <button
                            on:click={toggle_play}
                            class="btn btn-circle btn-sm"
                        >
                            <span class="material-icons"
                                >{is_playing ? "pause" : "play_arrow"}</span
                            >
                        </button>
                        <span
                            >{pretty_seconds(current_time)} / {pretty_seconds(
                                video_duration
                            )}</span
                        >
                    </div>

                    <div>
                        <VolumeController bind:value={volume} />
                        <button
                            on:click={() => (loop_clip = !loop_clip)}
                            class="btn btn-circle btn-sm"
                            class:spin={loop_clip}
                        >
                            <span class="material-icons">loop</span>
                        </button>

                        <button
                            on:click={toggle_fullscreen}
                            class="btn btn-circle btn-sm"
                        >
                            <span class="material-icons"
                                >{video_fullscreen
                                    ? "fullscreen_exit"
                                    : "fullscreen"}</span
                            >
                        </button>
                    </div>
                </div>

                <input
                    type="range"
                    min="0"
                    on:input={(e) =>
                        (video_player.currentTime =
                            e.currentTarget.valueAsNumber)}
                    bind:value={current_time}
                    max={video_duration}
                    class="range w-full range-xs"
                />
            </div>
        </div>
    {/if}
</div>
<div bind:clientHeight={timeline_height}>
    <div class="flex justify-between mx-3 my-2">
        <button on:click={() => move_one_frame(-1)} class="text-sm">
            <kbd class="kbd kbd-sm">◀︎</kbd> Previous frame
        </button>
        <button on:click={set_clip_start} class="text-sm">
            Set clip start <kbd class="kbd kbd-sm">▲</kbd>
        </button>
        <button on:click={timeline.center} class="text-sm">
            Center Timeline <kbd class="kbd kbd-sm">Space</kbd>
        </button>
        <button on:click={set_clip_end} class="text-sm"
            >Set clip end<kbd class="kbd kbd-sm">▼</kbd>
        </button>
        <button on:click={() => move_one_frame(1)} class="text-sm">
            Next frame <kbd class="kbd kbd-sm">▶︎</kbd>
        </button>
    </div>
    {#key video_duration}
        <Timeline
            bind:this={timeline}
            onUpdate={(e) => (video_player.currentTime = e)}
            video_current_time={current_time}
            seconds={video_duration}
        />
    {/key}
</div>

<style lang="scss">
    .video-player * {
        user-select: none;
    }

    .fullscreen {
        position: absolute;
        inset: 0;
        width: 100%;
    }

    .spin {
        animation: spin 2s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(-360deg);
        }
    }
</style>
