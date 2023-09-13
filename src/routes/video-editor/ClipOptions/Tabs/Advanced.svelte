<script lang="ts">
    import InputGroup from '$lib/components/InputGroup.svelte';
    import {
        audioBitrate,
        bitrateLock,
        calculatedAudioBitrate,
        calculatedVideoBitrate,
        finalBitrate,
        format,
        muteAudio,
        resolution,
        userBitrate,
        ValidationError,
        validationErrors
    } from '$lib/stores/ClipOptionsStore';

    interface Option {
        label: string
        value: number
        default?: boolean
    }

    const resolutionOptions: Option[] = [
        {
            label: '360p',
            value: 360
        },
        {
            label: '480p',
            value: 480
        },
        {
            label: '540p',
            value: 540
        },
        {
            label: '720p',
            value: 720,
            default: true
        },
        {
            label: '768p',
            value: 768
        },
        {
            label: '900p',
            value: 900
        },
        {
            label: '1080p',
            value: 1080
        },
        {
            label: '1440p',
            value: 1440
        },
        {
            label: '1800p',
            value: 1800
        },
        {
            label: '2160p',
            value: 2160
        },
        {
            label: '2880p',
            value: 2880
        },
        {
            label: '4320p',
            value: 4320
        }
    ];

    function updateBitrate(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        if ($bitrateLock) return;
        $userBitrate = e.currentTarget.valueAsNumber;
    }

    function updateAudioBitrate(
        e: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) {
        if ($muteAudio) return;
        $audioBitrate = e.currentTarget.valueAsNumber;
    }

    function toggleBitrateLock() {
        $bitrateLock = !$bitrateLock;
        $userBitrate = $calculatedVideoBitrate;
    }
</script>

<InputGroup label="Video Bitrate">
    <button on:click={toggleBitrateLock} disabled={$format.limitations.includes('NoAudio')}
            class="btn btn-square btn-sm">
        <i class="fas fa-lock" class:fa-lock={$bitrateLock} class:fa-lock-open={!$bitrateLock}/>
    </button>
    <input
            class:input-error={$validationErrors.includes(
            ValidationError.InvalidVideoBitrate
        )}
            disabled={$bitrateLock || $format.limitations.includes('NoBitrate')}
            type="number"
            class="input input-bordered input-sm w-full"
            on:input={updateBitrate}
            value={isNaN($finalBitrate) ? 0 : $finalBitrate.toFixed(2)}
    />
    <span>Kbits/s</span>
</InputGroup>

<InputGroup label="Audio Bitrate">
    <input
            disabled={$muteAudio || $format.limitations.includes('NoBitrate') || $format.limitations.includes('NoAudio')}
            title={$muteAudio
            ? "Cannot change audio bitrate when 'Mute Audio' option is selected."
            : ''}
            type="number"
            class="input input-bordered input-sm w-full"
            on:input={updateAudioBitrate}
            value={isNaN($calculatedAudioBitrate) ? 0 : $calculatedAudioBitrate}
    />
    <span>Kbits/s</span>
</InputGroup>

<InputGroup label="Resolution">
    <select class="select w-full select-bordered" bind:value={$resolution}>
        {#each resolutionOptions as option}
            <option value={option.value} selected={option.default}
            >{option.label}</option
            >
        {/each}
    </select>
</InputGroup>
