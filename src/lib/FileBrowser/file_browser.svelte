<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { directoryContents } from "../../stores";

  import { currentPath } from "../../stores";
  import { get } from "svelte/store";

  /**
   * @param {string} path
   */
  async function navigateToDirectory(path) {
    currentPath.set(path);
    try {
      const contents = await invoke("list_files_in_directory", { path });
      directoryContents.set(contents);
    } catch (error) {
      console.error("Error navigating to directory:", error);
    }
  }

  navigateToDirectory(get(currentPath));

  currentPath.subscribe((newPath) => {
    navigateToDirectory(newPath);
  });
</script>

<div>
  <h1>File Browser</h1>
  <ul>
    {#each $directoryContents as [name, isDir]}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
      <li
        on:click={() => isDir && navigateToDirectory(`${$currentPath}/${name}`)}
      >
        {name}
        {isDir ? "📁" : "📄"}
      </li>
    {/each}
  </ul>
</div>
