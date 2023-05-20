import FileSize from '../../shared/FileSize';
import { writable, derived } from 'svelte/store';

export const clip_name = writable("");
export const max_filesize = writable(8);
export const framerate = writable(24);
export const speed = writable(1);
export const user_bitrate = writable(1);
export const bitrate_lock = writable(false);

export const resolition = writable("1280x720");

export const clip_start = writable<number>(null);
export const clip_end = writable<number>(null);


export const duration = derived([speed, clip_start, clip_end], ([$speed, $clip_start, $clip_end]) => ($clip_end - $clip_start) / $speed);


export const calculated_bitrate = derived([duration, max_filesize], ([$duration, $max_filesize]) => {
    if($duration <= 0 ) return 0;
    return FileSize.fromMegaBytes($max_filesize).toKiloBits() / $duration

});

export const final_bitrate = derived([calculated_bitrate, user_bitrate, bitrate_lock], ([$calculated_bitrate, $user_bitrate, $bitrate_lock]) => {
    return $bitrate_lock ? $calculated_bitrate : $user_bitrate;
}) 


export const estimated_size = derived([final_bitrate, duration], ([$final_bitrate, $duration]) => FileSize.fromKiloBits($final_bitrate).bytes * $duration);
