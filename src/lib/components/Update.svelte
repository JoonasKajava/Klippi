<script lang="ts">
    import { getName, getVersion } from "@tauri-apps/api/app";
    import { installUpdate, type UpdateManifest } from "@tauri-apps/api/updater";
    import { relaunch } from '@tauri-apps/api/process'
    import { fly } from "svelte/transition";

    export let manifest: UpdateManifest;
    export let open: boolean;

    let name: string;
    let version: string;
    (async () => {
        name = await getName();
        version = await getVersion();
    })();

    function submit(
        e: Event & {
            readonly submitter: HTMLElement;
        } & {
            currentTarget: EventTarget & HTMLFormElement;
        }
    ) {
        e.preventDefault();
        let as_button = e.submitter as HTMLButtonElement;
        if(as_button.name === "yes") {
            installUpdate().then(() => {
                relaunch()
            });
        }else {
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
            {manifest?.version ?? "unknown"} is now available -- you have {version}.
        </p>
        <p>Would you like to install it now?</p>
        {#if manifest && manifest.body}
            <p>{manifest.body}</p>
        {/if}
        <div class="modal-action">
            <button name="no" class="btn btn-error btn-outline">No</button>
            <button name="yes" class="btn btn-success">Yes</button>
        </div>
    </form>
</dialog>
