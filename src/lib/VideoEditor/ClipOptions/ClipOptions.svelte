<script lang="ts">
    import type { ComponentType } from "svelte";
    import Basic from "./Tabs/Basic.svelte";
    import Advanced from "./Tabs/Advanced.svelte";
    import Share from "./Tabs/Share.svelte";
    import { fly } from "svelte/transition";
    import Stats from "./Stats.svelte";

    import { calculated_audio_bitrate, clip_end, clip_name, clip_start, final_bitrate, framerate, mute_audio, resolition, speed, validation_errors } from "./ClipOptionsStore";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { ClipCreationOptions } from "src/models/ClipCreationOptions";
    import { selected_video } from "../VideoEditorStore";
    import { current_page } from "../../shared/AppStore";
    import Processing from "../../Processing/Processing.svelte";


    let current_tab: ComponentType = Basic;

    let move_direction = 0;

    let tab_order = [Basic, Advanced, Share];

    function change_tab(to: ComponentType) {
        if (current_tab == to) return;
        move_direction =
            (tab_order.indexOf(current_tab) - tab_order.indexOf(to)) * -1;
        current_tab = to;
    }



    async function create_clip() {
        let name = $clip_name + ".mp4";
        let exists = await invoke<boolean>("clip_exists", {file: name});
        let confirmed = true;
        if(exists) {
            confirmed = await confirm(`Clip ${name} already exists, do you want to override it?`);
        }
        if(!confirmed) return;
        let options: ClipCreationOptions = {
            from: $selected_video,
            to: name,
            start: $clip_start,
            end: $clip_end,
            video_bitrate:$final_bitrate,
            audio_bitrate: $calculated_audio_bitrate,
            framerate: $framerate,
            speed: $speed,
            resolution: $resolition,
            mute: $mute_audio
        };

        invoke("create_clip", {options: options});

        $current_page = Processing;
    } 

</script>

<div class="px-3">
    <h2 class="text-2xl text-center mb-2">Create Clip</h2>
    <div class="tabs tabs-boxed flex-nowrap justify-center">
        <button
            class:tab-active={current_tab == Basic}
            on:click={() => change_tab(Basic)}
            class="tab flex-grow">Basic</button
        >
        <button
            class:tab-active={current_tab == Advanced}
            on:click={() => change_tab(Advanced)}
            class="tab flex-grow">Advanced</button
        >
        <button
            disabled={true}
            class:tab-active={current_tab == Share}
            on:click={() => change_tab(Share)}
            class="tab flex-grow">Sharing</button
        >
    </div>

    {#key current_tab}
        <div
            style="width: 300px;"
            in:fly={{ x: 100 * move_direction, duration: 200, delay: 200 }}
            out:fly={{ x: -100 * move_direction, duration: 200 }}
        >
            <svelte:component this={current_tab} />
            <button on:click={async () => await create_clip()} disabled={$validation_errors.length > 0} class="btn w-full my-2 btn-primary">Create Clip!</button>
            <Stats />
        </div>
    {/key}

    
</div>
