<script lang="ts">

    export let dependencies: string[] = [];

    interface DependencyInfo {
        name: string
        description: string
        source: string
    }

    const dependencyInfos: Record<string, DependencyInfo> = {
        ffprobe: {
            name: 'FFprobe',
            description:
                'Used to gather information about media, such as video duration',
            source: 'https://ffmpeg.org/'
        },
        ffmpeg: {
            name: 'FFmpeg',
            description:
                'Used to process media',
            source: 'https://ffmpeg.org/'
        }
    };

    $: dependencies_info = dependencies.map((x) => dependencyInfos[x]);
</script>

<div class="stats shadow">
    {#each dependencies_info as dependency}
        <div class="stat">
            <div class="stat-value text-xl">{dependency.name}</div>
            <div class="stat-desc max-w-xs whitespace-normal">{dependency.description}
                <br/>
                Source: <a href="{dependency.source}">{dependency.source}</a>
            </div>
        </div>
    {/each}
</div>
