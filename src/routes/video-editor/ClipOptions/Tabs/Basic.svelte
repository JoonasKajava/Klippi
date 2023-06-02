<script lang="ts">
    import InputGroup from "$lib/components/InputGroup.svelte";
    import { clip_name, validation_errors, bitrate_lock, max_file_size, framerate, speed, mute_audio, ValidationError } from "$lib/stores/ClipOptionsStore";


</script>

<InputGroup label="Clip Name">
    <input
        class:input-error={$validation_errors.includes(
            ValidationError.InvalidClipName
        )}
        type="text"
        class="input input-bordered input-sm flex-grow"
        bind:value={$clip_name}
    />
    <span>.mp4</span>
</InputGroup>

<InputGroup label="Max File Size">
    <input
        disabled={!$bitrate_lock}
        title="Cannot enforce 'Max File Size' while manual 'Video bitrate' is enabled."
        type="number"
        step="0.1"
        min="0"
        class="input input-bordered input-sm flex-grow"
        bind:value={$max_file_size}
    />
    <span>MB</span>
</InputGroup>

<InputGroup label="Framerate">
    <input
        class:input-error={$validation_errors.includes(
            ValidationError.InvalidFramerate
        )}
        type="number"
        min="1"
        class="input input-bordered input-sm flex-grow w-2"
        bind:value={$framerate}
    />
    <span class="whitespace-nowrap">Frames Per Second</span>
</InputGroup>

<InputGroup label="Speed" altLabelLeft="1 = normal speed. 2 = twice as fast.">
    <input
        class:input-error={$validation_errors.includes(
            ValidationError.InvalidSpeed
        )}
        type="number"
        min="0"
        step="0.1"
        class="input input-bordered input-sm flex-grow w-2"
        bind:value={$speed}
    />
</InputGroup>

<div class="form-control">
    <label class="label cursor-pointer">
        <span class="label-text">Mute Audio</span>
        <input
            type="checkbox"
            bind:checked={$mute_audio}
            class="checkbox checkbox-primary"
        />
    </label>
</div>
