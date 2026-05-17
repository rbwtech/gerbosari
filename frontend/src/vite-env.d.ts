/// <reference types="svelte" />
/// <reference types="vite/client" />

// Fallback module declaration in case the `svelte` package's bundled types
// don't get picked up by the TS server (e.g. fresh clone before npm install).
declare module '*.svelte' {
  import type { SvelteComponent } from 'svelte';
  export default class extends SvelteComponent<any, any, any> {}
}

interface ImportMetaEnv {
  readonly VITE_API_BASE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
