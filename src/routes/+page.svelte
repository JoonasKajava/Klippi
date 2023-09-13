<script lang="ts">
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';

    import { open } from '@tauri-apps/api/dialog';
    import { videoDir } from '@tauri-apps/api/path';
    import { fly } from 'svelte/transition';
    import { error } from '@sveltejs/kit';
    import type { VideoData } from '$lib/models/VideoData';
    import { goto } from '$app/navigation';
    import { selectedVideo } from '$lib/stores/VideoEditorStore';
    import { getMatches } from '@tauri-apps/api/cli';

    import { dependenciesHasBeenVerified, missingDependencies } from '$lib/stores/InstallerStore';
    import { clipEnd, clipStart } from "$lib/stores/ClipOptionsStore";

    (async () => {
        if ($dependenciesHasBeenVerified) return;
        const result = await invoke<string[]>('verify_dependencies');
        missingDependencies.set(result);
        if (result.length > 0) {
            await goto('/installer');
        } else {
            dependenciesHasBeenVerified.set(true);
        }
    })().catch((err) => {
        throw error(500, { message: err, title: 'Unable to verify dependencies' });
    });

    $: if ($dependenciesHasBeenVerified) {
        getMatches().then((matches) => {
            if (matches.args.source.value !== null && $selectedVideo === null) {
                selectVideo(matches.args.source.value as string);
            }
        }).catch(() => {
        });
    }

    const latestVideos = invoke<VideoData[]>('discover_videos', { count: 3 });

    latestVideos.catch((err: string) => {
        throw error(500, { message: err, title: 'Unable to load latest videos' });
    });

    function selectVideo(video: string): void {
        $clipStart = null;
        $clipEnd = null;
        selectedVideo.set(video);
        void goto('/video-editor');
    }

    async function openFileDialog(e: Event): Promise<void> {
        e.preventDefault();

        const extensions = await invoke<string[]>('get_supported_video_extensions');

        let dir: string = '~/';
        try {
            dir = await videoDir();
        } catch (err) {
        }
        const file = await open({
            multiple: false,
            defaultPath: dir,
            directory: false,
            title: 'Select A Video',
            filters: [
                {
                    name: 'Video',
                    extensions
                }
            ]
        });

        if (file !== null) selectVideo(file as string);
    }
</script>

<div class="flex justify-center flex-col items-center">
    <h1 class="text-4xl text-center">Select A Video</h1>
    <input
            on:click={openFileDialog}
            type="file"
            class="file-input file-input-bordered file-input-info w-full max-w-xs mt-8"
    />
    {#await latestVideos}
        <div
                in:fly={{ y: 100, duration: 200 }}
                out:fly={{ y: -100, duration: 200 }}
                class="mt-5"
        >
            <h3 class="font-bold text-center">Loading latest videos</h3>
            <progress class="progress w-56"/>
        </div>
    {:then videos}
        <div
                in:fly={{ y: 100, duration: 200, delay: 200 }}
                out:fly={{ y: -100, duration: 200 }}
                class="stats shadow mt-5 w-min"
        >
            {#each videos as video}
                <div
                        on:click={() => { selectVideo(video.file); }}
                        on:keypress={() => { selectVideo(video.file); }}
                        class="stat video-thumbnail relative cursor-pointer"
                >
                    <div class="stat-value flex justify-center">
                        <img alt="" src={convertFileSrc(video.thumbnail)}/>
                    </div>
                    <div class="stat-desc">{video.name}</div>
                    <div class="overlay transition-all">
                        <button class="btn btn-success open hidden">Select</button>
                    </div>
                </div>
            {/each}
        </div>
    {/await}
</div>

<style lang="scss">
  .video-thumbnail {
    &:hover .overlay {
      .btn {
        display: block;
      }

      background-color: rgba(0, 0, 0, 0.4);
    }

    .overlay {
      display: flex;
      justify-content: center;
      align-items: center;
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
    }
  }
</style>
