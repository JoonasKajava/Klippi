<script>
    import { fly } from "svelte/transition";
    import Alert from "../Alert.svelte";

    export let title = "";
    export let error = "";
</script>

<div class="text-center" transition:fly={{ x: -100, duration: 200 }}>
    <h1 class="text-3xl my-3">{title || "Unrecoverable Error Occured"}</h1>
    {#if error}
        <Alert {error} />
    {/if}
</div>
