import { type Writable, writable } from 'svelte/store';

export const selectedVideo: Writable<string | null> = writable(null);
