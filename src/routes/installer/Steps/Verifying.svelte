<script lang="ts">
    import Step from './Step.svelte';
    import type { FlyParams } from 'svelte/transition';

    export let transitions: {
        in: FlyParams
        out: FlyParams
    };
</script>

<Step transitions={transitions}>
    <h1 class="text-2xl">Verifying Installation</h1>
    <progress class="progress w-56"></progress>
</Step>
