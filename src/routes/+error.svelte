<script>
    import { fly } from "svelte/transition";
    import Alert from "$lib/components/Alert.svelte";
    import { page } from "$app/stores";

</script>

<div class="text-center" transition:fly={{ x: -100, duration: 200 }}>
    <h1 class="text-3xl my-3">{$page.error?.title || "Unrecoverable Error Occured"}</h1>
    <Alert error={$page.error.message} />
</div>
