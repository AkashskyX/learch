<script>
    import { createEventDispatcher } from 'svelte';
    import { currentPath } from '../../stores'; // Assuming this is a store that holds the current path
  
    const dispatch = createEventDispatcher();

    /**
   * @type {any[]}
   */
    let pathSegments;
  
    $: {
        pathSegments = $currentPath.split('/').filter(Boolean);
        console.log('currentPath updated:', $currentPath);
        console.log('pathSegments:', pathSegments);
    }
  
    /**
   * @param {number} index
   */
   function navigateTo(index) {
  let newPath;
  if (index === 0) {
    // If the root '/' is clicked, navigate to root
    newPath = '/';
  } else {
    // For other segments, construct the path to navigate to
    newPath = '/' + pathSegments.slice(0, index).join('/');
  }

  console.log("navigating to ", newPath);
  dispatch('navigate', newPath);
}
  </script>
  
  <nav aria-label="Breadcrumb" class=" rounded-sm py-2 px-4 inline-flex items-center space-x-2">
    <ol class="flex space-x-1">
      <!-- Always display the root '/' and make it clickable -->
      <li class="flex items-center">
        <a href="javascript:void(0);" on:click={() => navigateTo(0)}
          class="text-gray-800 font-medium text-sm leading-tight rounded-sm py-2 px-3 bg-gray-50">
          /
        </a>
      </li>
      {#each pathSegments as segment, index}
        <li class="flex items-center">
          {#if index < pathSegments.length - 1}
            <!-- Non-terminal segment -->
            <a href="javascript:void(0);" on:click={() => navigateTo(index + 1 )}
              class=" text-gray-800 font-medium text-sm leading-tight rounded-sm py-2 px-3 bg-gray-50">
              {segment}
            </a>
            <!-- Separator -->
            <span class="text-gray-500 mx-1">/</span>
          {:else}
            <!-- Terminal segment (current directory) -->
            <span class="bg-gray-100 text-gray-800 font-medium text-sm leading-tight rounded-sm py-2 px-3">
              {segment } 
            </span>
          {/if}
        </li>
      {/each}
    </ol>
  </nav>
  