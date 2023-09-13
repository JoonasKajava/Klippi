import { writable } from 'svelte/store';

export const ffmpegInstallLocation = writable('');
export const dependenciesHasBeenVerified = writable(false);
export const missingDependencies = writable<string[]>([]);
