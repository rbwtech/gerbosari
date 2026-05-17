<script context="module" lang="ts">
  /**
   * Column definition consumed by DataTable. Lives in module context so it
   * can be re-imported with `import type { ColumnDef } from ...` from list
   * pages - Svelte instance scripts forbid type exports.
   */
  export interface ColumnDef<R> {
    key: string;
    label: string;
    align?: 'left' | 'right' | 'center';
    /** Width hint - any valid Tailwind width class (e.g. 'w-24'). */
    width?: string;
    /** Custom cell renderer. Plain strings are escaped automatically by Svelte. */
    render?: (row: R) => string;
    /** Disable sorting for non-comparable columns (actions, thumbnails). */
    sortable?: boolean;
    /** Hide on mobile card layout - useful for action columns. */
    hideOnMobile?: boolean;
  }
</script>

<script lang="ts" generics="T">
  /**
   * Reusable sortable admin table with built-in loading, empty, and error
   * states. Renders as a table on `md+` and as a stack of cards on smaller
   * viewports so the four admin list pages share one responsive treatment.
   *
   * @template T row shape
   */
  import { createEventDispatcher } from 'svelte';
  import Skeleton from '../../components/ui/Skeleton.svelte';
  import EmptyState from '../../components/ui/EmptyState.svelte';
  import { ChevronUp, ChevronDown } from '../../components/icons';

  export let columns: ColumnDef<T>[];
  export let rows: T[] = [];
  export let loading = false;
  export let error: string | null = null;
  export let getId: (row: T) => string;
  /** Override the default empty-state title (rendered when not loading/error). */
  export let emptyTitle = 'Belum ada data';
  export let emptyDescription = 'Data akan tampil di sini setelah ditambahkan.';

  const dispatch = createEventDispatcher<{
    rowClick: { row: T };
    retry: void;
  }>();

  let sortKey: string | null = null;
  let sortDir: 'asc' | 'desc' = 'asc';

  // Local sort without mutating the input array. Keep stable insertion order
  // when no sort key is set so server-side ordering is preserved.
  $: sortedRows = (() => {
    if (!sortKey) return rows;
    const key = sortKey;
    const dir = sortDir === 'asc' ? 1 : -1;
    return [...rows].sort((a, b) => {
      const av = (a as Record<string, unknown>)[key];
      const bv = (b as Record<string, unknown>)[key];
      if (av == null && bv == null) return 0;
      if (av == null) return 1;
      if (bv == null) return -1;
      if (typeof av === 'number' && typeof bv === 'number') return (av - bv) * dir;
      return String(av).localeCompare(String(bv), 'id') * dir;
    });
  })();

  function toggleSort(col: ColumnDef<T>) {
    if (col.sortable === false) return;
    if (sortKey === col.key) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = col.key;
      sortDir = 'asc';
    }
  }

  function alignCls(a: ColumnDef<T>['align']): string {
    if (a === 'right') return 'text-right';
    if (a === 'center') return 'text-center';
    return 'text-left';
  }
</script>

{#if error}
  <EmptyState variant="error" title="Tidak dapat memuat data" description={error}>
    <button
      type="button"
      class="text-sm font-medium text-menoreh-700 hover:text-menoreh-900 underline underline-offset-4"
      on:click={() => dispatch('retry')}
    >
      Coba lagi
    </button>
  </EmptyState>
{:else if loading}
  <!-- Loading skeleton: render as a placeholder table on md+, cards below -->
  <div class="hidden md:block overflow-hidden rounded-lg border border-krem-200 bg-white">
    <table class="w-full text-sm">
      <thead class="bg-krem-50 border-b border-krem-200">
        <tr>
          {#each columns as col (col.key)}
            <th class="px-4 py-3 {alignCls(col.align)} text-xs font-semibold uppercase tracking-wider text-arang-600">
              {col.label}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each Array(6) as _, i (i)}
          <tr class="border-b border-krem-100 last:border-b-0">
            {#each columns as col (col.key)}
              <td class="px-4 py-3">
                <Skeleton class="h-4 {col.width ?? 'w-full max-w-[180px]'}" />
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="md:hidden space-y-3">
    {#each Array(4) as _, i (i)}
      <div class="rounded-lg border border-krem-200 bg-white p-4 space-y-2">
        <Skeleton class="h-4 w-3/4" />
        <Skeleton class="h-3 w-1/2" />
      </div>
    {/each}
  </div>
{:else if rows.length === 0}
  <EmptyState variant="empty" title={emptyTitle} description={emptyDescription}>
    <slot name="empty" />
  </EmptyState>
{:else}
  <!-- Desktop: table -->
  <div class="hidden md:block overflow-x-auto rounded-lg border border-krem-200 bg-white">
    <table class="w-full text-sm">
      <thead class="bg-krem-50 border-b border-krem-200">
        <tr>
          {#each columns as col (col.key)}
            <th
              scope="col"
              class="px-4 py-3 {alignCls(col.align)} {col.width ?? ''} text-xs font-semibold uppercase tracking-wider text-arang-600"
              aria-sort={sortKey === col.key ? (sortDir === 'asc' ? 'ascending' : 'descending') : undefined}
            >
              {#if col.sortable === false}
                <span>{col.label}</span>
              {:else}
                <button
                  type="button"
                  on:click={() => toggleSort(col)}
                  class="inline-flex items-center gap-1.5 uppercase tracking-wider text-xs font-semibold text-arang-600 hover:text-arang-900 focus:outline-none focus-visible:ring-2 focus-visible:ring-menoreh-500 rounded"
                >
                  {col.label}
                  {#if sortKey === col.key}
                    {#if sortDir === 'asc'}
                      <ChevronUp class="w-3.5 h-3.5" strokeWidth={2} aria-hidden="true" />
                    {:else}
                      <ChevronDown class="w-3.5 h-3.5" strokeWidth={2} aria-hidden="true" />
                    {/if}
                  {/if}
                </button>
              {/if}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each sortedRows as row (getId(row))}
          <tr
            class="border-b border-krem-100 last:border-b-0 hover:bg-krem-50/60 transition-colors"
            on:click={() => dispatch('rowClick', { row })}
          >
            {#each columns as col (col.key)}
              <td class="px-4 py-3 align-middle {alignCls(col.align)} text-arang-800">
                <slot name="cell" {row} {col}>
                  {#if col.render}
                    {@html col.render(row)}
                  {:else}
                    {(row as Record<string, unknown>)[col.key] ?? ''}
                  {/if}
                </slot>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <!-- Mobile: stack of cards -->
  <div class="md:hidden space-y-3">
    {#each sortedRows as row (getId(row))}
      <div class="rounded-lg border border-krem-200 bg-white p-4">
        <dl class="space-y-2">
          {#each columns.filter((c) => !c.hideOnMobile) as col (col.key)}
            <div class="flex items-start justify-between gap-3">
              <dt class="text-xs font-semibold uppercase tracking-wider text-arang-500">{col.label}</dt>
              <dd class="text-sm text-arang-800 text-right max-w-[60%] break-words">
                <slot name="cell" {row} {col}>
                  {#if col.render}
                    {@html col.render(row)}
                  {:else}
                    {(row as Record<string, unknown>)[col.key] ?? ''}
                  {/if}
                </slot>
              </dd>
            </div>
          {/each}
        </dl>
        {#if $$slots.mobileActions}
          <div class="mt-3 pt-3 border-t border-krem-100">
            <slot name="mobileActions" {row} />
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}
