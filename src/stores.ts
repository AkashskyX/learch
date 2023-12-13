import { writable, get } from 'svelte/store';

 export let currentPath = writable('/'); // Start from root


export let directoryContents = writable([]);