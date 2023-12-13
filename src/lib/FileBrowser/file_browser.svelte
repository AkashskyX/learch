<script>
  import { invoke } from "@tauri-apps/api/tauri";
  // @ts-ignore
  import Icon from "@iconify/svelte";

  import { currentPath, directoryContents, isInitialView } from "../../stores";
  
  import { push } from "svelte-spa-router";
  import { onMount } from "svelte";
  import Controls from "../Controls/controls.svelte";
  import Item from "../Item/item.svelte";

  onMount(async () => {
    navigateToDirectory("/");
  });

  /**
   * @param {string} path
   */
  async function navigateToDirectory(path) {
    currentPath.set(path);
    isInitialView.set(false); // Not in the initial view once we navigate
    try {
      const contents = await invoke("list_files_in_directory", { path });
      directoryContents.set(contents);
    } catch (error) {
      console.error("Error navigating to directory:", error);
    }
  }

  function goBack() {
    // If at the root, go back to the initial view
    if ($currentPath === "/") {
      push("/");
    } else {
      // Otherwise, navigate up one directory
      let pathArray = $currentPath.split("/").filter(Boolean);
      pathArray.pop();
      const newPath = pathArray.length === 0 ? "/" : "/" + pathArray.join("/");
      navigateToDirectory(newPath);
    }
  }

  $: if (!$isInitialView && $currentPath === "/") {
    navigateToDirectory("/");
  }
</script>

{#if $isInitialView}
  <div class="flex flex-col items-center justify-center mt-4"></div>
{:else}
  <!-- File Browser Grid View -->
  <div class="file-browser mt-4 p-4 bg-white">
    <div class="flex justify-between items-center"></div>
    <div class="grid grid-cols-6 gap-0">
      {#each $directoryContents as [name, isDir]}

      <Item
      {name}
      {isDir}
      path={`${$currentPath}/${name}`}
      on:navigate={event => navigateToDirectory(event.detail)}
    />
       
       
      {/each}
    </div>
  </div>
{/if}
<Controls on:navigateBack={goBack} />
<style>
  /* Tailwind CSS provides all the styling you need via utility classes; no additional CSS needed */
</style>
