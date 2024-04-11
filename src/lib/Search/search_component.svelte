<script>
    import { onMount, onDestroy } from 'svelte';
    import { writable } from 'svelte/store';
    import { invoke } from '@tauri-apps/api/tauri';
  import { index_path } from '../../stores';
  import SearchList from './search_list.svelte';

    let searchQuery = '';
    /**
   * @type {any[]}
   */
    let searchResults = [];
    let isSearching = false;
    const searchOverlayVisible = writable(false);

    const handleKeyDown = (/** @type {{ key: string; altKey: any; }} */ event) => {
        console.log("key");
        if (event.key === 'Escape') {
            console.log("esc");
            searchOverlayVisible.set(false);
        }

        if (event.key === 'f') {
            searchOverlayVisible.set(true);
        }
    };

    /**
   * @param {string} result
   */
    function handleClick(result) {
        
        // @ts-ignore
        const  path =  result.path;
        // @ts-ignore
        const is_dir = result.isdir
       
        if (!is_dir) {
            invoke('open_file', { path }).catch(console.error);
        } else {
            // Assuming you have a similar Rust function for opening directories
            invoke('open_folder', { path }).catch(console.error);
        }
    }

    const search = async () => {
        if (searchQuery.trim() === '') {
            searchResults = [];
            return;
        }

        isSearching = true;
        try {
            const results = await invoke('search_files', { query: searchQuery , indexPath: $index_path});
            searchResults = results;
            console.log(searchResults);
        } catch (error) {
            console.error('Search error:', error);
            searchResults = [];
        } finally {
            isSearching = false;
        }
    };

   // @ts-ignore
     $: if (searchQuery) search();

    onMount(() => {
        window.addEventListener('keydown', handleKeyDown);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', handleKeyDown);
    });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
{#if $searchOverlayVisible}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="fixed inset-0 bg-white bg-opacity-10 flex items-center justify-center p-4 backdrop-blur-3xl" on:click|self={() => searchOverlayVisible.set(false)}>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="  fixed top-52  w-[700px]" on:click|stopPropagation>
            <input
                type="text"
                autocapitalize="false"
             
                autocorrect="false"
                contextmenu="false"
                bind:value={searchQuery}
                placeholder="Search..."
                class="w-full bg-gray-100 px-5 py-3 mb-4   rounded-lg "
            />

            {#if isSearching}
            <p class="text-gray-500">Searching...</p>
        {:else}
            <SearchList {searchResults} />
        {/if}

          

            
        </div>
    </div>
{/if}

<style>
   
    
</style>
