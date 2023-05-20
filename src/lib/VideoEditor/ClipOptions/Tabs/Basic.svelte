<script lang="ts">
        import InputGroup from "../../../shared/Input/InputGroup.svelte";
    import { clip_name, framerate, max_filesize, speed } from "../ClipOptionsStore";


</script>

<InputGroup label="Clip Name">
    <input type="text" class="input input-bordered input-sm flex-grow" bind:value={$clip_name} />
    <span>.mp4</span>
</InputGroup>

<InputGroup label="Max Filesize">
    <input type="number" step="0.1" min="0" class="input input-bordered input-sm flex-grow" bind:value={$max_filesize} />
    <span>MB</span>
</InputGroup>

<InputGroup label="Framerate">
    <input type="number" min="1" class="input input-bordered input-sm flex-grow w-2" bind:value={$framerate} />
    <span class="whitespace-nowrap">Frames Per Second</span>
</InputGroup>

<InputGroup label="Speed" altLabelLeft="1 = normal speed. 2 = twice as fast.">
    <input type="number" min="0" step="0.1" class="input input-bordered input-sm flex-grow w-2" bind:value={$speed} />
</InputGroup>
