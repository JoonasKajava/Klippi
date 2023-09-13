<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';

    export let disabled: boolean = false;
    export let href: string;
</script>

<button
        class="join-item btn"
        {disabled}
        class:btn-active={$page.url.pathname === href}
        on:click={async () => { await goto(href); }}>
    <slot/>
</button
>

<style lang="scss">
  .btn {
    border-top-right-radius: 0 !important;
    border-top-left-radius: 0 !important;
  }
</style>
