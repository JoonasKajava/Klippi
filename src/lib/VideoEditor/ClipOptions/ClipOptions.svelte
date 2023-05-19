<script lang="ts">
    import type { ComponentType } from "svelte";
    import Basic from "./Tabs/Basic.svelte";
    import Advanced from "./Tabs/Advanced.svelte";
    import Share from "./Tabs/Share.svelte";
    import { fly } from "svelte/transition";

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
    <div class="tabs tabs-boxed flex-nowrap">
        <button
            class:tab-active={current_tab == Basic}
            on:click={() => change_tab(Basic)}
            class="tab">Basic</button
        >
        <button
            class:tab-active={current_tab == Advanced}
            on:click={() => change_tab(Advanced)}
            class="tab">Advanced</button
        >
        <button
            class:tab-active={current_tab == Share}
            on:click={() => change_tab(Share)}
            class="tab">Sharing</button
        >
    </div>

    {#key current_tab}
        <div
            in:fly={{ x: 100 * move_direction, duration: 200, delay: 200 }}
            out:fly={{ x: -100 * move_direction, duration: 200 }}
        >
            <svelte:component this={current_tab} />
        </div>
    {/key}
</div>
