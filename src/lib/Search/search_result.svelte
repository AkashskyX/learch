<script>
  import Icon from '@iconify/svelte';
    import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  
    /**
   * @type {{ is_dir: any; path: any; title: any; }}
   */
     export let resultObj;
    export let selected = false;


 
  
    async function handleResultClick() {
      if (resultObj.is_dir) {
        // Handle directory opening
        console.log('Open directory:', resultObj.path);
        // Invoke the Tauri command for opening a directory here
         await invoke('open_folder', { path: resultObj.path }).catch(console.error);
      } else {
        // Handle file opening
        console.log('Open file:', resultObj.path);
        // Invoke the Tauri command for opening a file here
        await invoke('open_file', { path: resultObj.path }).catch(console.error);
      }
    }
  </script>
  
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
  <div
  tabindex="0"
  class="result-item bg-gray-100 p-2 rounded-md cursor-pointer hover:bg-gray-200 transition-all duration-150"
  class:selected={selected}
  on:click={handleResultClick}



>

<div class="flex flex-row justify-between mx-2">


    <strong>{resultObj.title}</strong>


    {#if resultObj.is_dir }

    <Icon icon="material-symbols-light:folder-outline" width="35" />
    {:else}
    <Icon icon="bitcoin-icons:file-outline" width="35" />

    {/if}


  </div>
   
  </div>
  
  <style>
    .result-item {
        transition: background-color 0.3s, border-color 0.3s;
    }
  
    .selected {
        background-color: white;
        border: 2px solid rgba(107, 114, 128, 0.5); /* gray-500 with 50% opacity */
        z-index: 10;
    }
  
    /* Ensure that the border fades in nicely */
    .result-item:not(.selected) {
        border: 2px solid transparent; /* transparent border to prevent layout shift */
    }
  </style>
  