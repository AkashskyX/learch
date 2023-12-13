<script>
          import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import DiskComponent from './disk_component.svelte';
  import { push } from 'svelte-spa-router';

      let diskInfo = '';
      /**
   * @type {{ name: string; // Extracts only the disk name
  available: number; total: number; }[]}
   */
      let disks = []


      /**
   * @param {string} data
   */
   function parseDiskInfo(data) {
    return data.split('\n').map(line => {
        const [namePart, spacePart] = line.split(':');
        const [available, total] = spacePart.trim().split(' / ').map(s => parseFloat(s));
        return {
            name: namePart.split(' (')[0].trim(), // Extracts only the disk name
            available,
            total
        };
    }).filter(disk => {
        // Filter based on disk name or other criteria
        return disk.name === "Disk Macintosh HD"; // Example: Only include the main disk
    });
}

onMount(async () => {
        try {
            const response = await invoke('get_disk_info');
            diskInfo = response;

             disks = parseDiskInfo(diskInfo);

        } catch (error) {
            console.error('Error fetching disk info:', error);
        }
    });




</script>





<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="flex flex-col  items-start" on:click={()=>{push("/explore")}}>

    <h1 class="px-2 mb-3"> disk </h1>

  


<div class="flex flex-col justify-start items-start bg-gray-50 p-5  rounded-sm text-gray-500">


    



{#each disks as  disk}

<DiskComponent disk={disk}/>

 
{/each}

</div>

</div>
