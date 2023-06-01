<script lang="ts">
    import Step from "./Step.svelte";

    export let transitions;
</script>

<Step transitions={transitions}>
    <h1 class="text-2xl">Unpacking And Installing</h1>
    <progress class="progress w-56"></progress>
</Step>
