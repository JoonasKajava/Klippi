<script lang="ts">
    import InputGroup from '$lib/components/InputGroup.svelte';
    import type { OutputFormat } from '$lib/models/OutputFormat';
    import {
        bitrateLock,
        clipName,
        format,
        framerate,
        maxFileSize,
        muteAudio,
        speed,
        ValidationError,
        validationErrors
    } from '$lib/stores/ClipOptionsStore';
    import { invoke } from '@tauri-apps/api';

    const formats = invoke<OutputFormat[]>('get_output_formats');

    function closeFormatDropdown() {
        const dropdown = document.getElementById('format-dropdown');
        if (!dropdown) {
            console.error('Could not find format dropdown');
            return;
        }
        dropdown.removeAttribute('open');
    }

    function formatDropdownFocusout(e: FocusEvent) {
        const element = e.relatedTarget as HTMLElement;
        if (element?.classList.contains('format')) return;
        closeFormatDropdown();
    }
</script>

<InputGroup label="Clip Name">
    <input
            class:input-error={$validationErrors.includes(
            ValidationError.InvalidClipName
        )}
            type="text"
            class="input input-bordered input-sm flex-grow"
            bind:value={$clipName}
    />

    <details
            id="format-dropdown"
            on:focusout={formatDropdownFocusout}
            class="dropdown dropdown-bottom dropdown-end"
    >
        <summary class="btn btn-sm radius rounded-l-none lowercase"
        >.{$format.name}
            <i class="fa-solid fa-caret-down"/>
        </summary>
        <ul
                class="p-2 shadow menu dropdown-content bg-base-300 rounded-box w-52 z-50"
        >
            {#await formats then format_options}
                {#each format_options as option}
                    <li>
                        <button
                                class="format"
                                on:click={() => {
                                $format = option;
                                closeFormatDropdown();
                            }}
                        >
                            {option.name}
                        </button>
                    </li>
                {/each}
            {/await}
        </ul>
    </details>
</InputGroup>

<InputGroup label="Max File Size">
    <input
            disabled={!$bitrateLock || $format.limitations.includes('NoBitrate')}
            title="Cannot enforce 'Max File Size' while manual 'Video bitrate' is enabled."
            type="number"
            step="0.1"
            min="0"
            class="input input-bordered input-sm flex-grow"
            bind:value={$maxFileSize}
    />
    <span>MB</span>
</InputGroup>

<InputGroup label="Framerate">
    <input
            class:input-error={$validationErrors.includes(
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
            class:input-error={$validationErrors.includes(
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
                disabled={$format.limitations.includes('NoAudio')}
                type="checkbox"
                bind:checked={$muteAudio}
                class="checkbox checkbox-primary"
        />
    </label>
</div>

<style lang="scss">
  #format-dropdown {
    .fa-caret-down {
      transition: transform 0.2s;
    }

    &[open] {
      .fa-caret-down {
        transform: rotate(180deg);
      }
    }
  }
</style>
