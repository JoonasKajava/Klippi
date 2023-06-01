<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    export let disabled: boolean = false;
    export let href: string;
</script>

<button class="btn" disabled={disabled} class:btn-active={$page.url.pathname == href} on:click={() => goto(href)}><slot /></button>