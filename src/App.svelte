<script>
  import Router from 'svelte-spa-router'
  import FileBrowser from "./lib/FileBrowser/file_browser.svelte";
  // @ts-ignore
  import Main from "./lib/main.svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import { listen } from '@tauri-apps/api/event';
  import { appWindow } from '@tauri-apps/api/window';



  import { onDestroy, onMount } from 'svelte';
  import Settings from './lib/Settings/Settings.svelte';


  onMount(()=>{
 
 
   

})





  let progress = '';
 


  







  const routes = {
    "/": Main , 
    '/explore': FileBrowser ,
    "/settings": Settings

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