<script>
  import Router from 'svelte-spa-router'
  import FileBrowser from "./lib/FileBrowser/file_browser.svelte";
  import Main from "./lib/main.svelte";


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