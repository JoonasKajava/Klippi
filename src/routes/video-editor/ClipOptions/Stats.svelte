<script lang="ts">
    import { validation_errors, duration, estimated_size, ValidationError } from "$lib/stores/ClipOptionsStore";
    import prettyBytes from "pretty-bytes";

</script>

<div class="stats shadow">
    <div class="stat" >
        <div class="stat-title">Clip Duration:</div>
        <div class:text-error={$validation_errors.includes(ValidationError.InvalidDuration)} class="stat-value">{$duration.toFixed(1)}s</div>
        <div class="stat-desc">Seconds</div>
    </div>
    <div class="stat auto-rows-min" >
        <div class="stat-title">Estimated Size:</div>
        <div class="stat-value text-3xl">{prettyBytes(isFinite($estimated_size) && $estimated_size > 0 ? $estimated_size : 0)}</div>
    </div>
</div>
