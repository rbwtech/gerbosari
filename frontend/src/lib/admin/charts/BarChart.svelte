<script lang="ts">
  /**
   * Hand-rolled SVG bar chart supporting both horizontal and vertical layouts.
   *
   * - Horizontal: bars stack vertically, each row 28px tall with 12px gap.
   *   Suitable for categorical breakdowns where labels are long.
   * - Vertical: bars sit on a shared baseline. Each column auto-sizes to fill
   *   the available width. Suitable for time series (e.g. months).
   *
   * The chart auto-scales the longest bar to the available axis length so the
   * caller never needs to pre-normalize values.
   */

  interface Datum {
    label: string;
    value: number;
    color: string;
  }

  export let data: Datum[] = [];
  export let orientation: 'horizontal' | 'vertical' = 'horizontal';
  /** Unit suffix appended to value tooltips and aria labels (e.g. "Lowongan"). */
  export let unit: string = '';

  // Reactive max so the longest bar fills the axis. Min 1 prevents divide-by-zero
  // when every value is 0 (empty axis still renders cleanly).
  $: maxValue = Math.max(1, ...data.map((d) => Math.max(0, d.value)));
  $: hasData = data.some((d) => d.value > 0);
</script>

{#if hasData}
  {#if orientation === 'horizontal'}
    <!-- Horizontal: a flex column of label/track rows. Pure CSS handles the
         scaling; no SVG required because each bar is a single div whose width
         is a percentage of the row. This avoids text-on-SVG sizing quirks. -->
    <div class="flex flex-col gap-3">
      {#each data as datum}
        {@const pct = (Math.max(0, datum.value) / maxValue) * 100}
        {@const ariaLabel = `${datum.label}: ${datum.value.toLocaleString('id-ID')}${unit ? ' ' + unit : ''}`}
        <div
          class="flex flex-col gap-1"
          role="group"
          aria-label={ariaLabel}
        >
          <div class="flex items-baseline justify-between gap-3 text-sm">
            <span class="text-arang-700 truncate">{datum.label}</span>
            <span class="text-arang-900 font-medium tabular-nums shrink-0">
              {datum.value.toLocaleString('id-ID')}
            </span>
          </div>
          <div class="relative h-2.5 w-full rounded-full bg-krem-100 overflow-hidden">
            <div
              class="absolute inset-y-0 left-0 rounded-full transition-[width] duration-500 ease-out"
              style="width: {pct}%; background-color: {datum.color};"
            ></div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <!-- Vertical: SVG so we can lock the baseline + share a viewBox-aware grid.
         Each bar's height is a percentage of the chart-area height; labels sit
         below the baseline. 6 bars at ~14% width each leaves comfortable gaps. -->
    {@const barCount = data.length}
    {@const slotWidth = 100 / barCount}
    {@const barWidth = slotWidth * 0.55}
    <div class="flex flex-col gap-2">
      <div class="relative h-40 sm:h-48 w-full">
        <svg
          viewBox="0 0 100 100"
          preserveAspectRatio="none"
          class="w-full h-full"
          aria-hidden="true"
        >
          <!-- Gridlines at 25/50/75/100 of the y-axis for visual scanning. -->
          {#each [25, 50, 75] as y}
            <line x1="0" x2="100" y1={y} y2={y} stroke="#ebdfd0" stroke-width="0.3" />
          {/each}
          <line x1="0" x2="100" y1="100" y2="100" stroke="#c4ab8e" stroke-width="0.4" />

          {#each data as datum, i}
            {@const h = (Math.max(0, datum.value) / maxValue) * 95}
            {@const x = i * slotWidth + (slotWidth - barWidth) / 2}
            {@const y = 100 - h}
            {@const ariaLabel = `${datum.label}: ${datum.value.toLocaleString('id-ID')}${unit ? ' ' + unit : ''}`}
            <rect
              x={x}
              y={y}
              width={barWidth}
              height={h}
              fill={datum.color}
              rx="0.6"
              role="img"
              aria-label={ariaLabel}
            >
              <title>{datum.label}: {datum.value.toLocaleString('id-ID')}{unit ? ` ${unit}` : ''}</title>
            </rect>
          {/each}
        </svg>

        <!-- Value labels positioned via flex over the SVG. Avoids the SVG text
             scaling problem when preserveAspectRatio is set to "none". -->
        <div class="absolute inset-0 flex items-end pointer-events-none">
          {#each data as datum}
            {@const pct = (Math.max(0, datum.value) / maxValue) * 95}
            <div class="flex-1 flex flex-col items-center justify-end h-full">
              <span
                class="text-[10px] sm:text-xs font-medium tabular-nums text-arang-700"
                style="margin-bottom: calc({pct}% + 2px);"
                aria-hidden="true"
              >
                {datum.value > 0 ? datum.value.toLocaleString('id-ID') : ''}
              </span>
            </div>
          {/each}
        </div>
      </div>

      <div class="flex w-full">
        {#each data as datum}
          <div class="flex-1 text-center text-[10px] sm:text-xs text-arang-500 truncate px-0.5">
            {datum.label}
          </div>
        {/each}
      </div>
    </div>
  {/if}
{:else}
  <div class="w-full flex items-center justify-center py-10 text-sm text-arang-500">
    Belum ada data
  </div>
{/if}
