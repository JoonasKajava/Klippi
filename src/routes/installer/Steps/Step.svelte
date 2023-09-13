<script lang="ts">
    import { fly, type FlyParams } from 'svelte/transition';

    export let transitions: {
        in: FlyParams
        out: FlyParams
    };

    let isTransitioningOut = false;
</script>
<div class="mt-10" on:outrostart={() => { isTransitioningOut = true }} class:absolute={isTransitioningOut}
     in:fly={transitions.in} out:fly={transitions.out}>
    <slot></slot>
</div>
