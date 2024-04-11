<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Disk from "./Disk/disk.svelte";
  import { fade } from 'svelte/transition';

  import FileBrowser from "./FileBrowser/file_browser.svelte";
  import IndexInfo from "./Index/index_info.svelte";
  import SearchComponent from "./Search/search_component.svelte";
  import Settings from "./Settings/Settings.svelte";
  /**
   * @type {any}
   */
  let indexMetadata;
  let x = 0;
  let showSettings = true;

  let diskInfo = "";
  /**
   * @type {{ name: string; // Extracts only the disk name
  available: number; total: number; }[]}
   */
  let disks = [];

  /**
   * @param {string} data
   */
  function parseDiskInfo(data) {
    return data
      .split("\n")
      .map((line) => {
        const [namePart, spacePart] = line.split(":");
        const [available, total] = spacePart
          .trim()
          .split(" / ")
          .map((s) => parseFloat(s));
        return {
          name: namePart.split(" (")[0].trim(), // Extracts only the disk name
          available,
          total,
        };
      })
      .filter((disk) => {
        // Filter based on disk name or other criteria
        return disk.name === "Disk Macintosh HD"; // Example: Only include the main disk
      });
  }

 

  onMount(async () => {
    try {
      const response = await invoke("get_disk_info");
      diskInfo = response;

    

      disks = parseDiskInfo(diskInfo);
    } catch (error) {
      console.error("Error fetching disk info:", error);
    }

    setTimeout(()=>{

      showSettings = false

    } , 3000)
  });
</script>

<main class="h-screen font-mono">
  <div
    class="flex flex-col items-start justify-start p-3 bg-white text-black font-mono"
  >
    <h1 class="text-3xl mb-4 px-2">Learch</h1>

    <div class="flex flex-col">
      <Disk />
      <SearchComponent />
      
      <!-- Button to toggle the Settings component visibility -->
      <button
        class="mb-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700 transition duration-300 mt-10"
        on:click={() => showSettings = !showSettings}
      >
        Settings
      </button>

      <!-- Conditionally render the Settings component based on showSettings variable -->
      {#if showSettings}
      <div transition:fade>
        <Settings />
      </div>
    {/if}
    

    <div class="absolute bottom-10">
      <!-- Possibly other content -->
    </div>
  </div>
</main>
