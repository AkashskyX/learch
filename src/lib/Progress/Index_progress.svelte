<script>
    import { onMount, onDestroy } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/tauri";
  import { slide } from "svelte/transition";
  
    /**
   * @type {Promise<import("@tauri-apps/api/event").UnlistenFn>}
   */
    let unlistenPromise;
    let rootPath = "/Users";
    let indexedFiles = 0;
    let totalFiles = 1; // Initialize to 1 to avoid division by zero
    let currentDirectory = "";
    let percentage = 0;
  
    
    let visible =false

    
    
  
    async function startIndexing() {
      visible =true
      try {
        await invoke("create_and_index", { rootPath: rootPath });
        console.log("Files indexed successfully.");
        setTimeout(()=>{
          visible = false

        } , 1000)
        currentDirectory = rootPath; // This will update the DOM
      } catch (error) {
        console.error("Error indexing files:", error);

        visible = false

      }
    }
  
    onMount(() => {
      startIndexing();
      
       unlistenPromise = listen("index-progress", (event) => {
        const [indexedCountStr, totalCountStr] = event.payload.split(" / ");
        indexedFiles = parseInt(indexedCountStr, 10);
        totalFiles = parseInt(totalCountStr, 10) || 1; // Prevent division by zero
        percentage = (indexedFiles / totalFiles) * 100;
      });
  
     
    });


    onDestroy(async () => {
        const unlisten = await unlistenPromise;
        unlisten();
      });
  </script>
  
  
  

  {#if visible}

  <div in:slide  out:slide class="flex flex-col w-96 p-4 bg-gray-100 rounded-sm  space-y-2 font-mono">
    <div class="flex justify-between items-center">
      <h2 class="text-lg  text-gray-700">indexing</h2>
      <span class="text-2xl  text-gray-800 ">{percentage.toFixed(1)}%</span>
    </div>
    <div class="text-sm pb-2 text-gray-600">{rootPath}</div>
    <div class="w-full bg-gray-300 rounded-sm h-2.5">
      <div class="bg-blue-200 h-2.5 rounded-sm" style="width: {percentage}%;"></div>
    </div>
    <div class="text-sm text-gray-600">{indexedFiles} / {totalFiles} done</div>
  </div>
  

    
  {/if}