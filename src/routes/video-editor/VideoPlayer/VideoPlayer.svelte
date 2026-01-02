<script lang="ts">
    import {convertFileSrc} from '@tauri-apps/api/tauri';
    import {fly} from 'svelte/transition';
    import VolumeController from './VolumeController.svelte';
    import {appWindow} from '@tauri-apps/api/window';
    import Timeline from '../Timeline/Timeline.svelte';
    import {clipEnd, clipStart, duration, speed} from '$lib/stores/ClipOptionsStore';
    import {videoDuration} from '$lib/stores/VideoEditorStore';

    export let video: string;
    let timeline: Timeline;
    let videoPlayer: HTMLVideoElement;
    let isPlaying: boolean;
    let videoFullscreen = false;
    let displayControls = false;
    let loopClip = false;

    let currentTime: number = 0;
    let volume = 0.5;

    let timelineHeight: number;
    let windowHeight: number;

    const FRAME_RATE = 60;

    function moveOneFrame(direction: -1 | 1): void {
        videoPlayer.pause();
        videoPlayer.currentTime += direction / FRAME_RATE;
    }

    function togglePlay(): void {
        if (videoPlayer.paused) {
            videoPlayer.play().catch((e) => {
                console.error(e);
            });
        } else {
            videoPlayer.pause()
        }
        isPlaying = !videoPlayer.paused;
    }

    function timeupdate(): void {
        if (loopClip && $duration > 0 && $clipEnd && $clipStart) {
            if (
                videoPlayer.currentTime > $clipEnd ||
                videoPlayer.currentTime < $clipStart - 1
            ) {
                videoPlayer.currentTime = $clipStart;
            }
        }

        currentTime = videoPlayer.currentTime;
    }

    function prettySeconds(sec: number): string {
        return `${Math.floor(sec / 60)}:${(sec % 60)
            .toFixed(0)
            .padStart(2, '0')}`;
    }

    function toggleFullscreen(): void {
        appWindow.isFullscreen().then((value) => {
            appWindow.setFullscreen(!value).catch((e) => {
                console.error(e);
            });
            videoFullscreen = !value;
        }).catch((e) => {
            console.error(e);
        });
    }

    $: if (videoPlayer) {
        videoPlayer.volume = volume;
        videoPlayer.playbackRate = $speed;
    }

    function exitFullscreen(): void {
        appWindow.isFullscreen().then((value) => {
            if (value) toggleFullscreen();
        }).catch((e) => {
            console.error(e);
        });
    }

    function setClipStart(): void {
        $clipStart = videoPlayer.currentTime;
        timeline.update_clip_marker();
    }

    function setClipEnd(): void {
        $clipEnd = videoPlayer.currentTime;
        timeline.update_clip_marker();
    }

    function keyboardNavigation(
        e: KeyboardEvent & { currentTarget: EventTarget & Window }
    ): void {
        const targetAsElement = e.target as Element;
        if (targetAsElement && targetAsElement.nodeName.toLowerCase() === 'input') return;
        switch (e.code) {
            case 'ArrowLeft':
                moveOneFrame(-1);
                break;
            case 'ArrowRight':
                moveOneFrame(1);
                break;
            case 'ArrowUp':
                setClipStart();
                break;
            case 'ArrowDown':
                setClipEnd();
                break;
            case 'Space':
                timeline.center();
                break;
        }
    }

    function updateVideoMaxHeight(): void {
        const titleBar = document.getElementById('titlebar');
        if (!titleBar) return;
        videoPlayer.style.maxHeight = `${
            windowHeight -
            timelineHeight -
            titleBar.clientHeight -
            20
        }px`;
    }
</script>

<svelte:window
        on:resize={updateVideoMaxHeight}
        on:keyup={exitFullscreen}
        on:keydown={keyboardNavigation}
/>

<div
        class:fullscreen={videoFullscreen}
        on:mouseenter={() => (displayControls = true)}
        on:mouseleave={() => (displayControls = false)}
        class="relative inline-flex flex-col video-player transition-all z-50"
>
    <!-- svelte-ignore a11y-media-has-caption -->
    <video
            on:click={togglePlay}
            on:timeupdate={timeupdate}
            on:durationchange={() => ($videoDuration = videoPlayer.duration)}
            bind:this={videoPlayer}
    >
        <source src={convertFileSrc(video)}/>
    </video>
    {#if displayControls}
        <div
                transition:fly={{ y: 5, duration: 200 }}
                class="absolute inset-0 controls transition-all flex flex-col"
        >
            <div
                    class="grow"
                    on:click={togglePlay}
                    on:keypress={togglePlay}
            />

            <div
                    class=" self-end p-2 w-full bg-gradient-to-t from-black from-20% to-transparent text-white"
            >
                <div class="flex justify-between">
                    <div class="flex items-center gap-4">
                        <button
                                on:click={togglePlay}
                                class="btn btn-circle btn-sm"
                        >
                            <i class="fas fa-pause" class:fa-pause={isPlaying} class:fa-play={!isPlaying}></i>
                        </button>
                        <span
                        >{prettySeconds(currentTime)} / {prettySeconds(
                            $videoDuration
                        )}</span
                        >
                    </div>

                    <div>
                        <VolumeController bind:value={volume}/>
                        <button
                                on:click={() => (loopClip = !loopClip)}
                                class="btn btn-circle btn-sm"
                                class:spin={loopClip}
                        >
                            <i class="fa-solid fa-arrow-rotate-left"></i>
                        </button>

                        <button
                                on:click={toggleFullscreen}
                                class="btn btn-circle btn-sm"
                        >
                            <i class="fas fa-expand" class:fa-expand={!videoFullscreen}
                               class:fa-compress={videoFullscreen}></i>
                        </button>
                    </div>
                </div>

                <input
                        type="range"
                        min="0"
                        on:input={(e) =>
                        (videoPlayer.currentTime =
                            e.currentTarget.valueAsNumber)}
                        bind:value={currentTime}
                        max={$videoDuration}
                        class="range w-full range-xs"
                />
            </div>
        </div>
    {/if}
</div>
<div bind:clientHeight={timelineHeight}>
    <div class="flex justify-between mx-3 my-2">
        <button on:click={() => { moveOneFrame(-1); }} class="text-sm">
            <kbd class="kbd kbd-sm">◀︎</kbd> Previous frame
        </button>
        <button on:click={setClipStart} class="text-sm">
            Set clip start <kbd class="kbd kbd-sm">▲</kbd>
        </button>
        <button on:click={timeline.center} class="text-sm">
            Center Timeline <kbd class="kbd kbd-sm">Space</kbd>
        </button>
        <button on:click={setClipEnd} class="text-sm"
        >Set clip end<kbd class="kbd kbd-sm">▼</kbd>
        </button>
        <button on:click={() => { moveOneFrame(1); }} class="text-sm">
            Next frame <kbd class="kbd kbd-sm">▶︎</kbd>
        </button>
    </div>
    {#key $videoDuration}
        <Timeline
                bind:this={timeline}
                onUpdate={(e) => (videoPlayer.currentTime = e)}
                videoCurrentTime={currentTime}
                seconds={$videoDuration}
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
