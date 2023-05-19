import type { ComponentType } from 'svelte';
import { writable } from 'svelte/store';



function create_current_page_store() {
    const { subscribe, set } = writable<ComponentType>(null);

    return {
        subscribe,
        set: (new_page: ComponentType) => set(new_page)
    }
}
export const current_page = create_current_page_store();