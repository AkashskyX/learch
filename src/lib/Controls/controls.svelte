<!-- Controls.svelte -->
<script>
    import { createEventDispatcher, onMount } from 'svelte';
  
    const dispatch = createEventDispatcher();
  
    // Function to handle the goBack action
    function goBack() {
      dispatch('navigateBack');
    }
  
    // Function to handle the keyboard event for Cmd + "<"
    /**
   * @param {{ metaKey: any; key: string; }} event
   */
    function handleKeydown(event) {
      // Change 'Meta' to 'Control' if you want this to work on Windows/Linux
      if (event.metaKey && event.key === ',') {
        goBack();
      }
    }
  
    // Listen for the keydown event when the component mounts
    onMount(() => {
      window.addEventListener('keydown', handleKeydown);
  
      return () => {
        // Cleanup the event listener when the component is destroyed
        window.removeEventListener('keydown', handleKeydown);
      };
    });
  </script>
  
  <div class="controls text-xl h-15 px-6 py-2 bottom-0 fixed right-5 font-mono bg-white/30 backdrop-blur-sm rounded-full flex items-center space-x-4">
    <button
      class="shadow-none text-sm bg-blue-500 text-white px-3 py-1 rounded-full hover:bg-blue-600"
      on:click={goBack}
    >
      <span class="font-sans">⌘</span> + &lt;
    </button>
  
    <h1 class="text-lg">file_browser</h1>
  </div>
  
  