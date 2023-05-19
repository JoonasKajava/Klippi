<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import { fly } from "svelte/transition";
    import VolumeController from "./VolumeController.svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import Timeline from "../Timeline/Timeline.svelte";

    export let video: string;

    let video_player_container: HTMLElement;
    let video_player: HTMLVideoElement;
    let is_playing;
    let video_fullscreen = false;
    let display_controls = false;

    let duration: number;
    let current_time: number = 0;
    let volume = 0.5;

    function toggle_play() {
        video_player.paused ? video_player.play() : video_player.pause();
        is_playing = !video_player.paused;
    }

    function timeupdate() {
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
    }

    function exit_fullscreen() {
        appWindow.isFullscreen().then((value) => {
            if (value) toggle_fullscreen();
        });
    }

</script>

<svelte:window on:keyup={exit_fullscreen} />

<div
    class:fullscreen={video_fullscreen}
    on:mouseenter={() => (display_controls = true)}
    on:mouseleave={() => (display_controls = false)}
    class="relative inline-flex flex-col video-player w-5/12 transition-all"
>
    <!-- svelte-ignore a11y-media-has-caption -->
    <video
        on:click={toggle_play}
        on:timeupdate={timeupdate}
        on:durationchange={(d) => (duration = video_player.duration)}
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
                                duration
                            )}</span
                        >
                    </div>

                    <div>
                        <VolumeController bind:value={volume} />
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
                    max={duration}
                    class="range w-full range-xs"
                />
            </div>
        </div>
    {/if}
</div>

<Timeline seconds={duration} />

<style lang="scss">
    .video-player * {
        user-select: none;
    }

    .fullscreen {
        position: absolute;
        inset: 0;
        width: 100%;
    }
</style>
