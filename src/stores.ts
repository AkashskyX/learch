import { writable, get } from 'svelte/store';

 export let currentPath = writable('/'); // Start from root


export let directoryContents = writable([]);

// In your stores.js or similar file
export const isInitialView = writable(true);


export let atRoot =  writable(true)

export let index_path =  writable("/Users/sky/Documents/GitHub/learch/1")