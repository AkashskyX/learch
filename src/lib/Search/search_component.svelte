<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import {index_path} from '../../stores'

    let searchQuery = '';
    /**
   * @type {any[]}
   */
    let searchResults = [];
    let isSearching = false;


    const search = async () => {
        if (searchQuery.trim() === '') {
            searchResults = [];
            return;
        }

        isSearching = true;
        try {
            const results = await invoke('search_files', { query: searchQuery, indexPath: $index_path });
            searchResults = results;
        } catch (error) {
            console.error('Search error:', error);
            searchResults = [];
        } finally {
            isSearching = false;
        }
    };

    // Reactive statement to trigger search on every keystroke
    $: if (searchQuery) search();
</script>

<div class="search-container">
    <input type="text" bind:value={searchQuery} placeholder="Search files..." />

    {#if isSearching}
        <p>Searching...</p>
    {/if}

    <div class="results">
        {#each searchResults as result}
            <div class="result-item">
                {result}
            </div>
        {/each}
    </div>
</div>

<style>
    .search-container {
        background-color: white;
        padding: 10px;
    }

    input[type="text"] {
        width: 100%;
        padding: 8px;
        margin-bottom: 10px;
        border: 1px solid lightgrey;
        box-sizing: border-box;
    }

    .results {
        background-color: #f7f7f7;
        border-top: 1px solid lightgrey;
    }

    .result-item {
        padding: 8px;
        border-bottom: 1px solid lightgrey;
    }
</style>
