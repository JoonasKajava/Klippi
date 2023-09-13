<script lang="ts">
    import { duration, estimatedSize, format } from '$lib/stores/ClipOptionsStore';
    import prettyBytes from 'pretty-bytes';

    $: no_bitrate = $format.limitations.includes('NoBitrate');

    $: estimated_size_display = () => {
        if (no_bitrate) {
            return 'N/A';
        }
        return prettyBytes(
            isFinite($estimatedSize) && $estimatedSize > 0
                ? $estimatedSize
                : 0
        );
    };
</script>

<div class="stats shadow">
    <div class="stat">
        <div class="stat-title">Clip Duration:</div>
        <div
                class="stat-value"
        >
            {$duration.toFixed(1)}s
        </div>
        <div class="stat-desc">Seconds</div>
    </div>
    <div class="stat auto-rows-min">
        <div class="stat-title">Estimated Size:</div>
        <div class="stat-value text-3xl">{estimated_size_display()}</div>
        {#if no_bitrate}
            <div class="stat-desc whitespace-break-spaces">Cannot estimate size</div>
        {/if}
    </div>
</div>
