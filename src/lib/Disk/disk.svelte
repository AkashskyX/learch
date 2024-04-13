<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import DiskComponent from "./disk_component.svelte";
  import { push } from "svelte-spa-router";

  let diskInfo = "";
  /**
   * @type {{ name: string; available: number; total: number; }[]}
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
          name: namePart.split(" (")[0].trim(),
          available,
          total,
        };
      })
      .filter((disk) => {
        return disk.name === "Disk Macintosh HD"; // Filter to only include the main disk
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

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="fixed bottom-10 left-10 w-11/12 max-w-xs bg-gray-100 p-6 rounded-lg shadow-lg overflow-hidden"
  on:click={() => {
    push("/explore");
  }}>
  <div class="flex flex-row space-y-4">
    <div class="flex items-center pl-4 pr-8">
      <img src="/image.png" class="w-16 h-8 mr-3" alt="Disk Logo">
      <!-- <span class="text-lg font-bold">Disk Information</span> -->
    </div>
</div>
{#each disks as disk}
  <DiskComponent {disk} />
{/each}
</div> 