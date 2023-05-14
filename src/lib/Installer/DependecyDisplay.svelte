<script lang="ts">

    export let depencies: string[];
    interface DependecyInfo {
        name: string;
        description: string;
        source: string;
    }

    let dependecy_infos: { [name: string]: DependecyInfo } = {
        ffprobe: {
            name: "FFprobe",
            description:
                "Used to gather information about media, such as video duration",
            source: "https://ffmpeg.org/",
        },
    };

    $: dependencies_info = depencies.map((x) => dependecy_infos[x]);
</script>


<div class="stats shadow">
    {#each dependencies_info as dependecy}
        <div class="stat">
            <div class="stat-value text-xl">{dependecy.name}</div>
            <div class="stat-desc max-w-xs whitespace-normal">{dependecy.description}
                <br />
                Source: <a href="{dependecy.source}">{dependecy.source}</a>
            </div>
        </div>
    {/each}
</div>