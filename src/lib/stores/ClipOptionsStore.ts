import type { OutputFormat } from '$lib/models/OutputFormat';
import FileSize from '../utilities/FileSize';
import { writable, derived } from 'svelte/store';

export const clip_name = writable("");
export const max_file_size = writable(8);
export const framerate = writable(24);
export const speed = writable(1);
export const mute_audio = writable(false);

export const user_bitrate = writable(1);
export const bitrate_lock = writable(true);

export const audio_bitrate = writable(64);

export const resolution = writable(720);

export const clip_start = writable<number>(null);
export const clip_end = writable<number>(null);

export const format = writable<OutputFormat>({name: "mp4", extension: "mp4", limitations:[], preset: "ultrafast"});


export const duration = derived([speed, clip_start, clip_end], ([$speed, $clip_start, $clip_end]) => Math.max(($clip_end - $clip_start) / $speed, 0));

/// In kilobits
export const calculated_audio_bitrate = derived([audio_bitrate, mute_audio], ([$audio_bitrate, $mute_audio]) => {
    if ($mute_audio) return 0;
    return $audio_bitrate;

});
/// In kilobits
export const calculated_video_bitrate = derived([duration, max_file_size, calculated_audio_bitrate], ([$duration, $max_file_size, $calculated_audio_bitrate]) => {
    if ($duration <= 0) return 0;
    return (FileSize.fromMegaBytes($max_file_size).toKiloBits() / $duration) - $calculated_audio_bitrate

});



/// In kilobits
export const final_bitrate = derived([calculated_video_bitrate, user_bitrate, bitrate_lock], ([$calculated_video_bitrate, $user_bitrate, $bitrate_lock]) => {
    return ($bitrate_lock ? $calculated_video_bitrate : $user_bitrate);
})


export const estimated_size = derived([final_bitrate, duration, calculated_audio_bitrate], ([$final_bitrate, $duration, $calculated_audio_bitrate]) => FileSize.fromKiloBits($final_bitrate + $calculated_audio_bitrate).bytes * $duration);



export enum ValidationError {
    InvalidDuration,
    InvalidFramerate,
    InvalidSpeed,
    InvalidVideoBitrate,
    InvalidClipName
}

export const validation_errors = derived(
    [
        clip_name,
        duration,
        framerate,
        speed,
        final_bitrate
    ],
    ([
        clip_name,
        duration,
        framerate,
        speed,
        final_bitrate
    ]) => {
        let validation_errors: ValidationError[] = [];
        if (!clip_name) validation_errors.push(ValidationError.InvalidClipName);
        if (duration <= 0) validation_errors.push(ValidationError.InvalidDuration);
        if (framerate <= 0) validation_errors.push(ValidationError.InvalidFramerate);
        if (speed <= 0) validation_errors.push(ValidationError.InvalidSpeed);
        if (!isFinite(final_bitrate) || final_bitrate <= 1) validation_errors.push(ValidationError.InvalidVideoBitrate);
        return validation_errors;
    }
)