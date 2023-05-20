<script lang="ts">
    import InputGroup from "../../../shared/Input/InputGroup.svelte";
    import {
    ValidationError,
    audio_bitrate,
        bitrate_lock,
        calculated_audio_bitrate,
        calculated_video_bitrate,
        final_bitrate,
        mute_audio,
        resolition,
        user_bitrate,
        validation_errors,
    } from "../ClipOptionsStore";
    interface Option {
        label: string;
        value: string;
        default?: boolean;
    }

    const resolution_options: Option[] = [
        {
            label: "640x360",
            value: "640x360",
        },
        {
            label: "854x480",
            value: "854x480",
        },
        {
            label: "960x540",
            value: "960x540",
        },
        {
            label: "1280x720",
            value: "1280x720",
            default: true,
        },
        {
            label: "1366x768",
            value: "1366x768",
        },
        {
            label: "1600x900",
            value: "1600x900",
        },
        {
            label: "1920x1080",
            value: "1920x1080",
        },
        {
            label: "2560x1440",
            value: "2560x1440",
        },
        {
            label: "3200x1800",
            value: "3200x1800",
        },
        {
            label: "3840x2160",
            value: "3840x2160",
        },
        {
            label: "5120x2880",
            value: "5120x2880",
        },
        {
            label: "7680x4320",
            value: "7680x4320",
        },
    ];

    function update_bitrate(e: Event & {currentTarget: EventTarget & HTMLInputElement;}) {
        if($bitrate_lock) return;
        $user_bitrate = e.currentTarget.valueAsNumber;
    }

    function update_audio_bitrate(e: Event & {currentTarget: EventTarget & HTMLInputElement;}) {
        if($mute_audio) return;
        $audio_bitrate = e.currentTarget.valueAsNumber;
    }
    function toggle_bitrate_lock() {
        $bitrate_lock = !$bitrate_lock
        $user_bitrate = $calculated_video_bitrate;
    }
</script>

<InputGroup label="Video Bitrate">
    <button
        on:click={toggle_bitrate_lock}
        class="btn btn-square btn-sm"
    >
        <i class="material-icons">{$bitrate_lock ? "lock" : "lock_open"}</i>
    </button>
    <input
    class:input-error={$validation_errors.includes(
        ValidationError.InvalidVideoBitrate
    )}
        disabled={$bitrate_lock}
        type="number"
        class="input input-bordered input-sm w-full"
        on:input={update_bitrate}
        value={isNaN($final_bitrate) ? 0 : $final_bitrate.toFixed(2)}
    />
    <span>Kbits/s</span>
</InputGroup>

<InputGroup label="Audio Bitrate">
    <input
        disabled={$mute_audio}
        title={$mute_audio ? "Cannot change audio bitrate when 'Mute Audio' option is selected." : ""}
        type="number"
        class="input input-bordered input-sm w-full"
        on:input={update_audio_bitrate}
        value={isNaN($calculated_audio_bitrate) ? 0 : $calculated_audio_bitrate}
    />
    <span>Kbits/s</span>
</InputGroup>

<InputGroup label="Resolution">
    <select class="select w-full select-bordered" bind:value={$resolition}>
        {#each resolution_options as option}
            <option value={option.value} selected={option.default}
                >{option.label}</option
            >
        {/each}
    </select>
</InputGroup>
