<script>
    import { createEventDispatcher, onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/tauri";
    import Icon from "@iconify/svelte";


    /**
   * Truncates a filename while preserving the file extension.
   * @param {string} filename The full filename to truncate.
   * @param {number} maxLength The maximum length of the truncated filename.
   * @returns {string} The truncated filename with the extension.
   */
  function truncateFilename(filename, maxLength) {
    // Get the file extension
    const extension = filename.substring(filename.lastIndexOf('.') + 1);
    // Truncate the filename without the extension if it's too long
    if (filename.length > maxLength) {
      return filename.substring(0, maxLength - extension.length - 3) + '...' + extension;
    }
    return filename;
  }

    /**
   * @type {Element}
   */
    let imageElement;
  
    /**
   * @type {any}
   */
     export let name;
    /**
   * @type {any}
   */
     export let isDir;
    /**
   * @type {any}
   */
     export let path;

     let previewSrc = "";
  
    const dispatch = createEventDispatcher();
  
    async function handleClick() {
      if (isDir) {
        dispatch('navigate', path);
      } else {
        try {
          await invoke('open_file', { path });
        } catch (error) {
          console.error('Error opening file:', error);
        }
      }
    }

    // Check if the file is an image based on its extension
  /**
   * @param {string} fileName
   */
  function isImageFile(fileName) {
    return /\.(jpe?g|png|gif|bmp)$/i.test(fileName);
  }

  /**
   * @param {Iterable<number>} buffer
   */
 


  
  



    
  </script>
  
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
  bind:this={imageElement}
    class="directory-item flex flex-col items-center w-36 m-3 hover:bg-gray-50 rounded-sm cursor-pointer"
    on:click={handleClick}
  >
    {#if isDir}
      <Icon icon="arcticons:folder" width="60" />

    {:else if isImageFile(name) }

  
    
      <Icon icon="arcticons:image-resizer" width="60" />
    
    {:else}
      <Icon icon="arcticons:file" width="60" />
    {/if}
    <span class="mt-2 text-sm text-center font-thin font-mono">{truncateFilename(name, 30)}</span>
  </div>
  
  <style>
    /* Add specific styles here if needed */
  </style>
  