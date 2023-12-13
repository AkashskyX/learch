
<script>
    /**
   * @type {{ name: any; total: number; available: number; }}
   */
     export let disk
</script>



<h1>{disk.name}</h1>
 <div class="relative w-[500px]">
 <div  style="width: {disk.total};" class="h-2  bg-blue-200 rounded-sm">  </div>
 <div  style="width: { disk.total  - disk.available }px;" class="h-2  bg-red-200 absolute top-0  rounded-sm">  </div>
</div>
<div class="flex flex-row mt-2 text-sm">
    <span>{Math.round(disk.total  - disk.available) }/{Math.round(disk.total)} used </span>
</div>  