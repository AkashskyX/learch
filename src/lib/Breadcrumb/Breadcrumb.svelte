<script>
    import { createEventDispatcher } from 'svelte';
    import { currentPath } from '../../stores'; // Import the store or reactive variable

    /**
   * @type {any[]}
   */
    let pathSegments = [];
    const dispatch = createEventDispatcher();

    $: {
        pathSegments = $currentPath.split('/').filter(Boolean);
        console.log(pathSegments, "path: ", $currentPath);
    }

    /**
   * @param {number} segmentIndex
   */
    function navigateTo(segmentIndex) {
        const newPath = '/' + pathSegments.slice(0, segmentIndex + 1).join('/');
        dispatch('navigate', newPath);
    }
</script>

<nav aria-label="Breadcrumb" class="  py-2 px-4 inline-flex items-center space-x-2">
    <ol class="flex ">
      {#each pathSegments as segment, index}
        <li class="flex items-center">
          {#if index < pathSegments.length - 1}
            <!-- Non-terminal segment -->
            <!-- svelte-ignore a11y-invalid-attribute -->
            <a href="javascript:void(0);" on:click={() => navigateTo(index)}
               class="bg-white text-gray-800 font-medium text-sm leading-tight rounded-sm py-2 px-3 hover:bg-gray-100">
              {segment}
            </a> 
            <!-- Separator -->
            <span class="text-gray-500 mx-1">/</span>
          {:else}
            <!-- Terminal segment (current directory) -->
            <span class="bg-gray-100 text-gray-800 font-medium text-sm leading-tight rounded-sm py-2 px-3">
              {segment}
            </span>
          {/if}
        </li>
      {/each}
    </ol>
  </nav>
  
  
<style>
    /* Styles remain the same */
</style>
