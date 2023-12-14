<script>
  import Router from 'svelte-spa-router'
  import FileBrowser from "./lib/FileBrowser/file_browser.svelte";
  import Main from "./lib/main.svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';



  let rootPath = '/Users/sky/Documents/GitHub/learch';



  async function createAndIndexFiles() {
    try {
      await invoke('create_and_index', { rootPath: rootPath });
      console.log('Files indexed successfully.');
    } catch (error) {
      console.error('Error indexing files:', error);
    }
  }






onMount(()=>{
  createAndIndexFiles()
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