<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { appDataDir } from "@tauri-apps/api/path";
    import Step from "./Step.svelte";
    import { ffmpeg_install_location } from "../../../lib/stores/InstallerStore";

    export let onNext: () => void;

    async function get_default_install_directory() {
        return await appDataDir();
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

    let ffmpeg_install_location_input : HTMLInputElement;

    function next() {
        $ffmpeg_install_location = ffmpeg_install_location_input.value;
        onNext();
    }

    export let transitions;
</script>

<Step transitions={transitions}>
    <div class="form-control">
        <span class="label">FFmpeg & FFProbe Installation Directory</span>

        <div class="input-group">
            <input
                bind:this={ffmpeg_install_location_input}
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
        <button on:click={next} class="btn btn-outline btn-success">
            Next
            <i class="fas fa-chevron-right"></i>
        </button>
    </div>
</Step>
