import { writable } from 'svelte/store';

export const ffmpeg_install_location = writable("");
export const dependencies_has_been_verified = writable(false);
export const missing_dependencies = writable<string[]>([]);