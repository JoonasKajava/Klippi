<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  import { videoDir } from "@tauri-apps/api/path";
  import { fly } from "svelte/transition";
  import { error } from "@sveltejs/kit";
  import type { VideoData } from "../lib/models/VideoData";
  import { goto } from "$app/navigation";
  import { selected_video } from "$lib/stores/VideoEditorStore";
    import { dependencies_has_been_verified, missing_dependencies } from "$lib/stores/InstallerStore";

  (async () => {
    if ($dependencies_has_been_verified) return;
    let result = await invoke<string[]>("verify_dependencies");
    missing_dependencies.set(result);
    if (result.length > 0) {
      goto("/installer");
    }
  })().catch((err) => {
    throw error(500, { message: err, title: "Unable to verify dependencies" });
  })

  let latest_videos = invoke<VideoData[]>("get_latest_videos", { count: 3 });

  latest_videos.catch((err: string) => {
    throw error(500, { message: err, title: "Unable to load latest videos" });
  });

  function select_video(video: string) {
    selected_video.set(video);
    goto("/video-editor");
  }

  async function open_file_dialog(e: Event) {
    e.preventDefault();
    const file = await open({
      multiple: false,
      defaultPath: await videoDir(),
      directory: false,
      title: "Select A Video",
      filters: [
        {
          name: "Video",
          extensions: ["mp4"],
        },
      ],
    });

    if (file) select_video(file as string);
  }
</script>

<div class="flex justify-center flex-col items-center">
  <h1 class="text-4xl text-center">Select A Video</h1>
  <input
    on:click={open_file_dialog}
    type="file"
    class="file-input file-input-bordered file-input-info w-full max-w-xs mt-8"
  />
  {#await latest_videos}
    <div
      in:fly={{ y: 100, duration: 200 }}
      out:fly={{ y: -100, duration: 200 }}
      class="mt-5"
    >
      <h3 class="font-bold text-center">Loading latest videos</h3>
      <progress class="progress w-56" />
    </div>
  {:then videos}
    <div
      in:fly={{ y: 100, duration: 200, delay: 200 }}
      out:fly={{ y: -100, duration: 200 }}
      class="stats shadow mt-5 w-min"
    >
      {#each videos as video}
        <div
          on:click={() => select_video(video.file)}
          on:keypress={() => select_video(video.file)}
          class="stat video-thumbnail relative cursor-pointer"
        >
          <div class="stat-value flex justify-center">
            <img alt="" src={convertFileSrc(video.thumbnail)} />
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
