<script lang="ts">
    import prettyBytes from "pretty-bytes";
    import { duration, estimated_size } from "./ClipOptionsStore";

</script>

<div class="stats shadow">
    <div class="stat" >
        <div class="stat-title">Clip Duration:</div>
        <div class="stat-value">{$duration.toFixed(1)}s</div>
        <div class="stat-desc">Seconds</div>
    </div>
    <div class="stat" >
        <div class="stat-title">Estimated Size:</div>
        <div class="stat-value text-3xl">{prettyBytes(isNaN($estimated_size) ? 0 : $estimated_size)}</div>
    </div>
</div>
