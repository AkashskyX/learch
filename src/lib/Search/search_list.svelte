<script>
    import { onMount, onDestroy } from 'svelte';
    import SearchResult from '../Search/search_result.svelte';
  import { invoke } from '@tauri-apps/api';

    /**
   * @type {string | any[]}
   */
     export let searchResults = [];
    let selectedIndex = 0;

    /**
   * @param {{ is_dir: any; path: any; }} resultObj
   */
    async function handleResultClick(resultObj) {
      if (resultObj.is_dir) {
        // Handle directory opening
        console.log('Open directory:', resultObj.path);
        // Invoke the Tauri command for opening a directory here
         await invoke('open_directory', { path: resultObj.path }).catch(console.error);
      } else {
        // Handle file opening
        console.log('Open file:', resultObj.path);
        // Invoke the Tauri command for opening a file here
        await invoke('open_file', { path: resultObj.path }).catch(console.error);
      }
    }

    const handleKeyDown = (/** @type {{ key: any; preventDefault: () => void; }} */ event) => {
        switch (event.key) {
            case 'ArrowDown':
                event.preventDefault();
                if (selectedIndex < searchResults.length - 1) {
                    selectedIndex++;
                }
                break;
            case 'ArrowUp':
                event.preventDefault();
                if (selectedIndex > 0) {
                    selectedIndex--;
                }
                break;
            case 'Enter':
            handleResultClick(JSON.parse(searchResults[selectedIndex]));
                break;
        }
    };

    onMount(() => {
        window.addEventListener('keydown', handleKeyDown);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', handleKeyDown);
    });

    
</script>

<div class="search-results-container">
    {#each searchResults as result, index}
        {@const resultObj = JSON.parse(result)}
        <div class="search-result-item">
            <SearchResult
                {resultObj}
                selected={index === selectedIndex}
                
            />
        </div>
    {/each}
</div>



<style>
    /* This ensures no horizontal scrolling */
    .search-results-container {
        max-height: 96vh; /* Adjust to your preference */
        overflow-y: auto; /* Only vertical scroll */
        overflow-x: hidden; /* No horizontal scroll */
        padding-right: 0.5rem; /* Adjust as needed */
        padding-left: 0.5rem; /* Adjust as needed */
    }

    /* This resets any default styles that might cause unexpected offsets */
    .search-result-item {
        margin: 0;
        padding: 0.5rem; /* Adjust as needed */
        list-style: none; /* Removes default list item styling */
    }
</style>
