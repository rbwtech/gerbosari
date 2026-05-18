<script lang="ts">
  /**
   * Hand-rolled SVG donut chart. Renders as a stack of stroked circle arcs
   * computed via `stroke-dasharray` offsets, which keeps the geometry to a
   * single math pass per segment and avoids any external chart dependency.
   *
   * Sized via Tailwind utility classes on the host (e.g. `w-36 sm:w-56`) so
   * mobile/desktop sizes are responsive without invalidating the viewBox math.
   */

  interface Segment {
    label: string;
    value: number;
    color: string;
  }

  export let data: Segment[] = [];
  export let centerValue: string = '';
  export let centerLabel: string = '';

  // r=40 inside a 100x100 viewBox keeps a comfortable 10-unit margin so the
  // stroke (width 14) doesn't get clipped at the edges.
  const CIRCUMFERENCE = 2 * Math.PI * 40;

  $: total = data.reduce((sum, s) => sum + Math.max(0, s.value), 0);

  // Pre-compute each segment's dash array + offset. We track a running
  // offset so arcs render contiguously around the ring. SVG strokes start at
  // 3 o'clock; the parent rotates -90deg so the ring visually starts at 12.
  $: segments = (() => {
    if (total <= 0) return [];
    let consumed = 0;
    return data.map((segment) => {
      const fraction = Math.max(0, segment.value) / total;
      const length = fraction * CIRCUMFERENCE;
      const offset = -consumed;
      consumed += length;
      return {
        dasharray: `${length} ${CIRCUMFERENCE - length}`,
        dashoffset: offset,
        color: segment.color,
        label: segment.label,
        value: segment.value,
        pct: fraction * 100
      };
    });
  })();
</script>

<div class="flex flex-col items-center gap-4">
  {#if total > 0}
    <div class="relative w-36 h-36 sm:w-56 sm:h-56">
      <svg
        viewBox="0 0 100 100"
        class="w-full h-full -rotate-90"
        role="img"
        aria-label={centerLabel ? `${centerValue} ${centerLabel}` : 'Donut chart'}
      >
        <!-- Track ring (subtle krem fill behind the segments) -->
        <circle cx="50" cy="50" r="40" fill="none" stroke="#f5ebe0" stroke-width="14" />
        {#each segments as seg}
          <circle
            cx="50"
            cy="50"
            r="40"
            fill="none"
            stroke={seg.color}
            stroke-width="14"
            stroke-dasharray={seg.dasharray}
            stroke-dashoffset={seg.dashoffset}
            stroke-linecap="butt"
          >
            <title>{seg.label}: {seg.value.toLocaleString('id-ID')} ({seg.pct.toFixed(1)}%)</title>
          </circle>
        {/each}
      </svg>
      <div
        class="absolute inset-0 flex flex-col items-center justify-center text-center pointer-events-none"
      >
        <span class="font-serif text-2xl sm:text-3xl font-semibold text-arang-900 tabular-nums leading-none">
          {centerValue}
        </span>
        {#if centerLabel}
          <span class="mt-1 text-[10px] sm:text-xs font-medium uppercase tracking-widest text-arang-500">
            {centerLabel}
          </span>
        {/if}
      </div>
    </div>

    <ul class="w-full flex flex-col gap-1.5 text-sm">
      {#each segments as seg}
        <li class="flex items-center justify-between gap-3">
          <span class="inline-flex items-center gap-2 min-w-0">
            <span
              class="inline-block w-2.5 h-2.5 rounded-sm shrink-0"
              style="background-color: {seg.color};"
              aria-hidden="true"
            ></span>
            <span class="text-arang-700 truncate">{seg.label}</span>
          </span>
          <span class="text-arang-900 font-medium tabular-nums shrink-0">
            {seg.value.toLocaleString('id-ID')}
            <span class="text-arang-500 font-normal ml-1">({seg.pct.toFixed(1)}%)</span>
          </span>
        </li>
      {/each}
    </ul>
  {:else}
    <div class="w-full flex items-center justify-center py-10 text-sm text-arang-500">
      Belum ada data
    </div>
  {/if}
</div>
