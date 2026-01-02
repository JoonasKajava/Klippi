<script lang="ts">
    import { getName, getVersion } from '@tauri-apps/api/app';
    import { installUpdate, type UpdateManifest } from '@tauri-apps/api/updater';
    import { relaunch } from '@tauri-apps/api/process'
    import { fly } from 'svelte/transition';

    export let manifest: UpdateManifest | undefined;
    export let open: boolean;

    let name: string;
    let version: string;
    void (async () => {
        name = await getName();
        version = await getVersion();
    })();

    function submit(
        e: Event & {
            readonly submitter: HTMLElement | null
        } & {
            currentTarget: EventTarget & HTMLFormElement
        }
    ): void {
        e.preventDefault();
        const asButton = e.submitter as HTMLButtonElement | null;
        if (asButton && asButton.name === 'yes') {
            void installUpdate().then(() => {
                void relaunch()
            });
        } else {
            open = false;
        }
    }
</script>

<dialog
        transition:fly={{ y: -100, duration: 200 }}
        id="update-available"
        class="modal"
        {open}
>
    <form on:submit={submit} method="dialog" class="modal-box">
        <h3 class="font-bold text-lg">A new version of {name} is available!</h3>
        <p class="py-4">
            {name}
            {manifest?.version ?? 'unknown'} is now available -- you have {version}.
        </p>
        <p>Would you like to install it now?</p>
        {#if manifest?.body}
            <p>{manifest.body}</p>
        {/if}
        <div class="modal-action">
            <button name="no" class="btn btn-error btn-outline">No</button>
            <button name="yes" class="btn btn-success">Yes</button>
        </div>
    </form>
</dialog>
