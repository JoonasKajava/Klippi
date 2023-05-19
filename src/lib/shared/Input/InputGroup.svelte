<script lang="ts">
    export let label = null;
    export let altLabelLeft = null;

</script>

<div class="form-control">
    {#if label}
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text">{label}</span>
        </label>
    {/if}
    <label class="input-group input-group-sm">
        <slot></slot>
    </label>
    {#if altLabelLeft}
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="label">
            <span class="label-text-alt">{altLabelLeft}</span>
        </label>
    {/if}
</div>
