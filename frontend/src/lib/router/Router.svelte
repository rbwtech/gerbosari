<script lang="ts">
  import type { ComponentType, SvelteComponent } from 'svelte';
  import { location } from './location';

  type AnyComponent = ComponentType<SvelteComponent>;

  /**
   * Path pattern → component. Special keys:
   *   ':name'  matches a single non-slash segment, exposed as params.name
   *   '*'      wildcard catch-all (used as 404 fallback, must be last)
   */
  export let routes: Record<string, AnyComponent>;

  type Spec = {
    raw: string;
    pattern: RegExp;
    paramNames: string[];
    component: AnyComponent;
  };

  function compile(path: string, component: AnyComponent): Spec {
    const paramNames: string[] = [];
    const escaped = path
      .replace(/[.+?^${}()|[\]\\]/g, '\\$&')
      .replace(/\/:([a-zA-Z_]+)/g, (_, name) => {
        paramNames.push(name);
        return '/([^/]+)';
      });
    return {
      raw: path,
      pattern: new RegExp(`^${escaped}$`),
      paramNames,
      component,
    };
  }

  $: specs = Object.entries(routes)
    .filter(([p]) => p !== '*')
    .map(([p, c]) => compile(p, c));

  $: wildcard = routes['*'] ?? null;

  $: matched = (() => {
    for (const spec of specs) {
      const m = $location.match(spec.pattern);
      if (m) {
        const params: Record<string, string> = {};
        spec.paramNames.forEach((name, i) => {
          params[name] = decodeURIComponent(m[i + 1] ?? '');
        });
        return { component: spec.component, params };
      }
    }
    return wildcard ? { component: wildcard, params: {} } : null;
  })();
</script>

{#if matched}
  <svelte:component this={matched.component} params={matched.params} />
{/if}
