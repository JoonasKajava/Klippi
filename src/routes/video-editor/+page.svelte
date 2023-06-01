<script lang="ts">
    import { selected_video } from "$lib/stores/VideoEditorStore";
    import { fly } from "svelte/transition";
    import ClipOptions from "./ClipOptions/ClipOptions.svelte";
    import VideoPlayer from "./VideoPlayer/VideoPlayer.svelte";
</script>

<div class="flex flex-grow">
    <div class="basis-2/3">
        <VideoPlayer video={$selected_video} />
    </div>
    <div in:fly={{delay: 200, y: 100, duration: 200}}  class="basis-1/3">
        <ClipOptions />
    </div>
</div>
