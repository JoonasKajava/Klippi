<script lang="ts">
  import TitleBar from "./lib/TitleBar.svelte";
  import Installer from "./lib/Installer/Installer.svelte";
  import { fly } from "svelte/transition";
  import { current_page } from "./lib/shared/AppStore";
  import Processing from "./lib/Processing/Processing.svelte";

  $current_page = Installer;
</script>

<TitleBar />
{#key $current_page}
  <div
    class="flex-grow"
    in:fly={{ delay: 200, duration: 200, y: 100 }}
    out:fly={{ duration: 200, y: -100 }}
  >
    <svelte:component this={$current_page} />
  </div>
{/key}
