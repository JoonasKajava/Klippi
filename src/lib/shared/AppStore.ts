import type { ComponentType, ComponentProps } from 'svelte';
import { writable } from 'svelte/store';
import type PanicRoom from '../PanicRoom/PanicRoom.svelte';
import type { SvelteComponent, SvelteComponentTyped } from 'svelte/internal';


function create_current_page_store() {
    const { subscribe, set } = writable<ComponentType>(null);
    return {
        subscribe,
        set: (new_page: ComponentType) => {
            set(new_page);
            current_page_props.set({});
        },

        setWithProps: <C extends SvelteComponent>(new_page: ComponentType, props: ComponentProps<C>) => {
            set(new_page);
            current_page_props.set(props);
        }
    }
}
export const current_page = create_current_page_store();
export const current_page_props = writable<any>({});