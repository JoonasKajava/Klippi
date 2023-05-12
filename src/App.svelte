<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
  import TitleBar from './lib/TitleBar.svelte';

  let latest_videos = invoke<string[]>("get_latest_videos",  {count: 3});
</script>

<TitleBar />

{#await latest_videos}
  <p>Loading</p>
{:then videos} 
  {#each videos as video}
    <div>{video}</div>
  {/each}
{/await}