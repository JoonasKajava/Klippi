import { writable } from 'svelte/store';

function createThumbnailProcesses() {
    const { update, subscribe } = writable<string[]>([]);

    return {
        subscribe,
        add: (process: string) => {
            update(list => {
                list.push(process);
                return list;
            });
        },
        remove: (process: string) => {
            update(list => {
                const index = list.indexOf(process);
                list.splice(index, 1);
                return list;
            });
        }
    }
}

export const thumbnailProcesses = createThumbnailProcesses();
