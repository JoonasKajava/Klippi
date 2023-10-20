import type {OutputFormat} from '$lib/models/OutputFormat';
import FileSize from '../utilities/FileSize';
import {derived, writable} from 'svelte/store';
import {videoDuration} from '$lib/stores/VideoEditorStore';

export const clipName = writable('');
export const maxFileSize = writable(8);
export const framerate = writable(24);
export const speed = writable(1);
export const muteAudio = writable(false);

export const userBitrate = writable(1);
export const bitrateLock = writable(true);

export const audioBitrate = writable(64);

export const resolution = writable(720);

export const clipStart = writable<number | null>(null);
export const clipEnd = writable<number | null>(null);

export const format = writable<OutputFormat>({ name: 'mp4', extension: 'mp4', limitations: [], preset: 'ultrafast' });

export const duration = derived([speed, clipStart, clipEnd, videoDuration], ([$speed, $clipStart, $clipEnd, $videoDuration]) => {
    if ($clipStart === null || $clipEnd === null) {
        return $videoDuration
    }
    return Math.max(($clipEnd - $clipStart) / $speed, 0);
});

/// In kilobits
export const calculatedAudioBitrate = derived([audioBitrate, muteAudio], ([$audioBitrate, $muteAudio]) => {
    if ($muteAudio) return 0;
    return $audioBitrate;
});
/// In kilobits
export const calculatedVideoBitrate = derived([duration, maxFileSize, calculatedAudioBitrate], ([$duration, $maxFileSize, $calculatedAudioBitrate]) => {
    if (!$duration || $duration <= 0) return 0;
    return (FileSize.fromMegaBytes($maxFileSize).toKiloBits() / $duration) - $calculatedAudioBitrate
});

/// In kilobits
export const finalBitrate = derived([calculatedVideoBitrate, userBitrate, bitrateLock], ([$calculatedVideoBitrate, $userBitrate, $bitrateLock]) => {
    return ($bitrateLock ? $calculatedVideoBitrate : $userBitrate);
})

export const estimatedSize = derived([finalBitrate, duration, calculatedAudioBitrate], ([$finalBitrate, $duration, $calculatedAudioBitrate]) => {
    if (!$duration) return 0;
    return FileSize.fromKiloBits($finalBitrate + $calculatedAudioBitrate).bytes * $duration
});

export enum ValidationError {
    InvalidFramerate,
    InvalidSpeed,
    InvalidVideoBitrate,
    InvalidClipName
}

export const validationErrors = derived(
    [
        clipName,
        framerate,
        speed,
        finalBitrate
    ],
    ([
         clipName,
         framerate,
         speed,
         finalBitrate
     ]) => {
        const validationErrors: ValidationError[] = [];
        if (!clipName) validationErrors.push(ValidationError.InvalidClipName);
        if (framerate <= 0) validationErrors.push(ValidationError.InvalidFramerate);
        if (speed <= 0) validationErrors.push(ValidationError.InvalidSpeed);
        if (!isFinite(finalBitrate) || finalBitrate <= 1) validationErrors.push(ValidationError.InvalidVideoBitrate);
        return validationErrors;
    }
)
