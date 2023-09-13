<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
    import { appDataDir } from '@tauri-apps/api/path';
    import Step from './Step.svelte';
    import { ffmpegInstallLocation } from '../../../lib/stores/InstallerStore';
    import type { FlyParams } from 'svelte/transition';

    export let onNext: () => void;

    async function getDefaultInstallDirectory() {
        return await appDataDir();
    }

    let installLocation: string;
    getDefaultInstallDirectory().then((x) => {
        installLocation = x;
    }).catch(console.error);

    async function selectFfmpegInstallationDirectory() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: installLocation
        });
        if (selected) {
            installLocation = selected as string;
        }
    }

    let ffmpegInstallLocationInput: HTMLInputElement;

    function next() {
        $ffmpegInstallLocation = ffmpegInstallLocationInput.value;
        onNext();
    }

    export let transitions: {
        in: FlyParams
        out: FlyParams
    };
</script>

<Step transitions={transitions}>
    <div class="form-control">
        <span class="label">FFmpeg & FFProbe Installation Directory</span>

        <div class="input-group">
            <input
                    bind:this={ffmpegInstallLocationInput}
                    readonly
                    type="text"
                    placeholder="Installation Directory..."
                    class="input input-bordered"
                    value={installLocation}
            />
            <button on:click={selectFfmpegInstallationDirectory} class="btn">
                Select Folder
            </button>
        </div>
        <label class="label cursor-pointer">
            <span class="label-text">Add Installation Directory Into PATH</span>
            <input type="checkbox" checked={true} class="checkbox"/>
        </label>
    </div>
    <div class="text-right mt-6">
        <button on:click={next} class="btn btn-outline btn-success">
            Next
            <i class="fas fa-chevron-right"></i>
        </button>
    </div>
</Step>
