
<script lang="ts">
    import { slide } from "svelte/transition";
    export let value: number;


    let display_volume_control = false;
    </script>

<div
    on:mouseenter={() => (display_volume_control = true)}
    on:mouseleave={() => (display_volume_control = false)}
    class="btn btn-circle btn-sm w-auto min-w-8 group"
>
<i class="fas fa-volume-up"></i>
    {#if display_volume_control}
        <input
            transition:slide={{ axis: "x", duration: 200 }}
            type="range"
            min="0"
            max="1"
            step="0.01"
            bind:value={value}
            class="range w-40 range-xs mx-2"
        />
    {/if}
</div>
