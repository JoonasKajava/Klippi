

<script type="ts">
    import { fly} from "svelte/transition";

    export let transitions;

    let is_transitioning_out = false;
</script>
<div class="mt-10" on:outrostart={() => is_transitioning_out = true} class:absolute={is_transitioning_out} in:fly={transitions.in} out:fly={transitions.out}>
    <slot></slot>
</div>