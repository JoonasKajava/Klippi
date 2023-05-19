<script lang="ts">
        import type { Options } from "../../..//shared/Input/Select.svelte";
        import Select from "../../..//shared/Input/Select.svelte";
import InputGroup from "../../../shared/Input/InputGroup.svelte";

const resolution_options: Options[] = [
    {
        label: "testing",
        value: "tesfdg"
    },
    {
        label: "testing",
        value: "tesfdg"
    },
    {
        label: "testing",
        value: "tesfdg"
    },
    {
        label: "testing",
        value: "tesfdg"
    }
];
</script>

<InputGroup label="Clip Name">
    <input type="text" class="input input-bordered input-sm flex-grow" />
    <span>.mp4</span>
</InputGroup>

<InputGroup label="Video Bitrate">
    <button class="btn btn-square btn-sm">
        <i class="material-icons">lock</i>
    </button>
    <input type="number" class="input input-bordered input-sm" />
    <span>Kbits/s</span>
</InputGroup>

<InputGroup label="Max Filesize">
    <input type="number" class="input input-bordered input-sm flex-grow" />
    <span>MB</span>
</InputGroup>

<InputGroup label="Framerate">
    <input type="number" min="1" class="input input-bordered input-sm flex-grow w-2" />
    <span class="whitespace-nowrap">Frames Per Second</span>
</InputGroup>

<InputGroup label="Speed" altLabelLeft="1 = normal speed. 2 = twice as fast.">
    <input type="number" min="0.001" class="input input-bordered input-sm flex-grow w-2" />
</InputGroup>

<Select options={resolution_options} />