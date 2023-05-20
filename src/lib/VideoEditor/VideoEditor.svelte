<script lang="ts">
    import ClipOptions from "./ClipOptions/ClipOptions.svelte";
import { selected_video } from "./VideoEditorStore";
    import VideoPlayer from "./VideoPlayer/VideoPlayer.svelte";
</script>
<div class="flex flex-grow">
    <div class="flex-grow">
        <VideoPlayer video={$selected_video} />
    </div>
    <div class="flex-shrink">
        <ClipOptions />
    </div>
</div>

