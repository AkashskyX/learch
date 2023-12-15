<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

    let segments = 0;
    let totalDocuments = 0;
    let sizeBytes = 0;
    let indexPath = '';
    /**
   * @type {string}
   */
    let path

    /**
   * @type {string}
   */
    let formattedSize;

   
  
    async function fetchIndexMetadata() {
      try {
         path = "/Users/sky/Documents/GitHub/learch/1";
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
    <div class="bg-white border border-gray-300 rounded-lg p-4 ">
        <h2 class="text-xl font-semibold mb-2">Index Information</h2>
        <p><span class="font-semibold">Number of Segments:</span> {segments}</p>
        <p><span class="font-semibold">Total Documents:</span> {totalDocuments}</p>
        <p><span class="font-semibold">Size (bytes):</span> {formattedSize}</p>
        <p><span class="font-semibold">Index Path:</span> {path}</p>
      
      </div>
      
   
  </div>
  