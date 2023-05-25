<script lang="ts">
    import ClipOptions from "./ClipOptions/ClipOptions.svelte";
import { selected_video } from "./VideoEditorStore";
    import VideoPlayer from "./VideoPlayer/VideoPlayer.svelte";
</script>
<div class="flex flex-grow">
    <div class="basis-2/3">
        <VideoPlayer video={$selected_video} />
    </div>
    <div class="basis-1/3">
        <ClipOptions />
    </div>
</div>

