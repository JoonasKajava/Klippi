<script lang="ts">
    import { open, ask } from "@tauri-apps/api/dialog";
    import { appDataDir, join } from "@tauri-apps/api/path";

    export let onNext: () => void;

    async function get_default_install_directory() {
        return await join(await appDataDir(), "ffmpeg");
    }

    let install_location: string;
    get_default_install_directory().then((x) => {
        install_location = x;
    });
    async function select_ffmpeg_installation_directory() {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: install_location
        });
        if(selected) {
            install_location = selected as string;
        }

    }
</script>

<div>
    <div class="form-control">
        <span class="label">FFmpeg & FFProbe Installation Directory</span>

        <div class="input-group">
            <input
                readonly
                type="text"
                placeholder="Installation Directory..."
                class="input input-bordered"
                value={install_location}
            />
            <button on:click={select_ffmpeg_installation_directory} class="btn">
                Select Folder
            </button>
        </div>
        <label class="label cursor-pointer">
            <span class="label-text">Add Installation Directory Into PATH</span>
            <input type="checkbox" checked={true} class="checkbox" />
        </label>
    </div>
    <div class="text-right mt-6">
        <button on:click={onNext} class="btn btn-outline btn-success">
            Next
            <span class="material-icons">arrow_forward_ios</span>
        </button>
    </div>
</div>
