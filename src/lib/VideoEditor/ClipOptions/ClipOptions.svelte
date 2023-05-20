<script lang="ts">
    import type { ComponentType } from "svelte";
    import Basic from "./Tabs/Basic.svelte";
    import Advanced from "./Tabs/Advanced.svelte";
    import Share from "./Tabs/Share.svelte";
    import { fly } from "svelte/transition";
    import Stats from "./Stats.svelte";
    import { clip_name, duration, final_bitrate, framerate, speed, validation_errors } from "./ClipOptionsStore";


    let current_tab: ComponentType = Basic;

    let move_direction = 0;

    let tab_order = [Basic, Advanced, Share];

    function change_tab(to: ComponentType) {
        if (current_tab == to) return;
        move_direction =
            (tab_order.indexOf(current_tab) - tab_order.indexOf(to)) * -1;
        current_tab = to;
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
            <button disabled={$validation_errors.length > 0} class="btn w-full my-2 btn-primary">Create Clip!</button>
            <Stats />
        </div>
    {/key}

    
</div>
