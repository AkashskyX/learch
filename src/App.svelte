<script>
  import Router from 'svelte-spa-router'
  import FileBrowser from "./lib/FileBrowser/file_browser.svelte";
  import Main from "./lib/main.svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';



  async function initializeSearchIndex() {
  try {
    await invoke('create_search_index');
    console.log('Search index created successfully.');
  } catch (error) {
    console.error('Error creating search index:', error);
  }
}


onMount(()=>{
  initializeSearchIndex();
})



  const routes = {
    "/": Main , 
    '/explore': FileBrowser
}

</script>

<body class="h">
  <Router {routes}/>
</body>



<style>
  .h{
    height: 100%;
    background: #ffffff;
  }
</style>