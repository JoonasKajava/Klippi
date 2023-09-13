<script lang="ts">
    import { selectedVideo } from '$lib/stores/VideoEditorStore';
    import { fly } from 'svelte/transition';
    import ClipOptions from './ClipOptions/ClipOptions.svelte';
    import VideoPlayer from './VideoPlayer/VideoPlayer.svelte';
</script>
{#if $selectedVideo}
    <div class="flex flex-grow">
        <div class="basis-2/3">
            <VideoPlayer video={$selectedVideo}/>
        </div>
        <div in:fly={{ delay: 200, y: 100, duration: 200 }} class="basis-1/3">
            <ClipOptions/>
        </div>
    </div>
{/if}
