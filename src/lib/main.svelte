<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Disk from "./Disk/disk.svelte";
  import FileBrowser from "./FileBrowser/file_browser.svelte";
  import IndexProgress from "./Progress/Index_progress.svelte";
  import IndexInfo from "./Index/index_info.svelte";
  import SearchComponent from "./Search/search_component.svelte";
  /**
   * @type {any}
   */
  let indexMetadata;
  let x = 0;

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
  });
</script>

<main class="h-screen font-mono">
  <div
    class="flex flex-col items-start justify-start p-3 bg-white  text-black font-mono"
  >
    <h1 class="text-3xl mb-4 px-2">Learch</h1>

    <div class=" flex flex-row">
   

      <Disk />


      <SearchComponent/>



      
    <div class="absolute bottom-10">


       <div class="mb-5">  <IndexInfo/> </div>
        <IndexProgress/>
    </div>
    </div>
  </div>
</main>
