<script lang="ts">
    import prettyBytes from "pretty-bytes";
    import { ValidationError, duration, estimated_size, validation_errors } from "./ClipOptionsStore";

</script>

<div class="stats shadow">
    <div class="stat" >
        <div class="stat-title">Clip Duration:</div>
        <div class:text-error={$validation_errors.includes(ValidationError.InvalidDuration)} class="stat-value">{$duration.toFixed(1)}s</div>
        <div class="stat-desc">Seconds</div>
    </div>
    <div class="stat" >
        <div class="stat-title">Estimated Size:</div>
        <div class="stat-value text-3xl">{prettyBytes(isFinite($estimated_size) ? $estimated_size : 0)}</div>
    </div>
</div>
