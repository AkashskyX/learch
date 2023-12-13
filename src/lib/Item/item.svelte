<script>
    import { createEventDispatcher, onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/tauri";
    import Icon from "@iconify/svelte";

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
  function arrayBufferToBase64(buffer) {
    return btoa(
      new Uint8Array(buffer)
        .reduce((data, byte) => data + String.fromCharCode(byte), '')
    );
  }

  async function loadImagePreview() {
    if (!isDir && isImageFile(name)) {
      try {
        const imageBytes = await invoke('thumbnail_generate', { path });

       
        previewSrc = `data:image/png;base64,${arrayBufferToBase64(imageBytes)}`;
      } catch (error) {
        console.error('Error loading image preview:', error);
      }
    }
  }


  onMount(() => {
    if (!isDir) {
        const observer = new IntersectionObserver(entries => {
      // Logic for when the image enters the viewport
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          // Load the image only if it is in the viewport
          loadImagePreview();
          // Stop observing the image if it's been loaded
          observer.unobserve(imageElement);
        }
      });
    });

    observer.observe(imageElement);

    return () => {
      observer.disconnect();
    };
    }
  });

  



    
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

  
      <!-- Image preview -->
      <img  src={previewSrc} alt={name} class="thumbnail" />
      <!-- <Icon icon="arcticons:image-resizer" /> -->
    
    {:else}
      <Icon icon="arcticons:file" width="60" />
    {/if}
    <span class="mt-2 text-sm text-center font-thin font-mono">{name}</span>
  </div>
  
  <style>
    /* Add specific styles here if needed */
  </style>
  