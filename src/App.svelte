<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import TitleBar from "./lib/TitleBar.svelte";
  import VideoSelector from "./lib/VideoSelector.svelte";
    import MissingDependencies from "./lib/Installer/MissingDependencies.svelte";
    import Installer from "./lib/Installer/Installer.svelte";

  let dependency_verification = invoke("verify_dependencies");
</script>

<TitleBar />

{#await dependency_verification}
  <div>Checking dependecies</div>
{:then _}
  <VideoSelector />
{:catch error}
  <Installer missing_dependencies={error} />
{/await}
