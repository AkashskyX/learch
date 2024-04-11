<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import {index_path} from '../../stores'
  import IndexProgress from "../Progress/Index_progress.svelte";


    let segments = 0;
    let totalDocuments = 0;
    let sizeBytes = 0;
    let indexPath = '';
    let isIndexing = false;
    /**
   * @type {string}
   */
    let path

    /**
   * @type {string}
   */
    let formattedSize;


    function reindex() {
    isIndexing = true; 
  }


    async function deleteIndex() {
    try {
      await invoke('delete_index_command');
      console.log('Index deleted');
      fetchIndexMetadata()
    } catch (error) {
      console.error('Failed to delete index:', error);
    }
  }

   
  
    async function fetchIndexMetadata() {

      segments = 0;
    totalDocuments = 0;
    sizeBytes = 0;
      try {

         path = await invoke("get_index_path")

         $index_path = path
        // path = "/Users/sky/Documents/GitHub/learch/1";
        console.log("Sending", path);
  
        const metadata = await invoke('get_index_metadata', { indexPath: path });
  
        // Split the metadata string by line
        const lines = metadata.split('\n');
  
        // Process each line to extract relevant information
        lines.forEach((/** @type {string} */ line) => {
          if (line.startsWith('Segment')) {
            const match = /Segment (\d+): (\d+) documents/.exec(line);
            if (match) {
              segments++;
              totalDocuments += parseInt(match[2], 10);
            }
          } else if (line.startsWith('Directory Size:')) {
            const match = /Directory Size: (\d+) bytes/.exec(line);
            if (match) {
              sizeBytes = parseInt(match[1], 10);
            }
          } else if (line.startsWith('Index Path:')) {
            indexPath = line.replace('Index Path: ', '');
         
          }
        });

        if (sizeBytes < 1024) {
  formattedSize = sizeBytes + ' bytes';
} else if (sizeBytes < 1024 * 1024) {
  formattedSize = (sizeBytes / 1024).toFixed(2) + ' KB';
} else if (sizeBytes < 1024 * 1024 * 1024) {
  formattedSize = (sizeBytes / (1024 * 1024)).toFixed(2) + ' MB';
} else {
  formattedSize = (sizeBytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
}



      } catch (error) {
        console.error("Failed to fetch index metadata:", error);
      }
    }
  
    onMount(()=>{
        
fetchIndexMetadata()
        

    })
  </script>
  
  <div>
    <div class="bg-white border border-gray-300 rounded-lg p-4 relative">
        <h2 class="text-xl font-semibold mb-2">Index Information</h2>
        <button 
          class="absolute top-3 right-3 text-lg" 
          on:click={fetchIndexMetadata} 
          title="Refresh Metadata"
        >
          🔄
        </button>
        <p><span class="font-semibold">Number of Segments:</span> {segments}</p>
        <p><span class="font-semibold">Total Documents:</span> {totalDocuments}</p>
        <p><span class="font-semibold">Size:</span> {formattedSize}</p>
        <p><span class="font-semibold">Index Path:</span> {path}</p>
  
        <button class="shadow-none border-gray-900 mt-5 " on:click={deleteIndex}>Delete</button>

        <button class="shadow-none border-gray-900 mt-5 " on:click={reindex}>re-index</button>
    </div>
    {#if isIndexing}
    <IndexProgress/>
      
    {/if}

  </div>