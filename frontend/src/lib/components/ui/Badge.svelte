<script lang="ts">
  /**
   * Semantic category badge. Variants map to content domains so routes can
   * pass the raw category string without translating it into colors.
   */
  export type BadgeVariant =
    | 'wisata'
    | 'budaya'
    | 'agrowisata'
    | 'kegiatan'
    | 'umkm'
    | 'formal'
    | 'freelance'
    | 'aktif'
    | 'tutup'
    | 'arsip'
    | 'berita'
    | 'agenda'
    | 'neutral';

  export let variant: BadgeVariant = 'neutral';
  /** Optional label prop - overridden by default slot if content provided. */
  export let label = '';

  let cls = '';
  export { cls as class };

  const variants: Record<BadgeVariant, string> = {
    // Domain categories
    wisata: 'bg-menoreh-100 text-menoreh-800',
    budaya: 'bg-terakota-100 text-terakota-800',
    agrowisata: 'bg-tanah-100 text-tanah-800',
    kegiatan: 'bg-krem-200 text-arang-700',

    // Lowongan kerja tiers - complementary tones, no palette clash
    umkm: 'bg-terakota-50 text-terakota-700',
    formal: 'bg-menoreh-50 text-menoreh-700',
    freelance: 'bg-tanah-50 text-tanah-700',

    // Status states
    aktif: 'bg-menoreh-500/10 text-menoreh-800',
    tutup: 'bg-arang-100 text-arang-700',
    arsip: 'bg-krem-100 text-arang-700',

    // Editorial
    berita: 'bg-terakota-50 text-terakota-700',
    agenda: 'bg-menoreh-50 text-menoreh-700',

    neutral: 'bg-krem-100 text-arang-700'
  };

  const base =
    'inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded text-xs font-medium uppercase tracking-wider';

  $: composed = `${base} ${variants[variant]} ${cls}`;
</script>

<span class={composed}>
  {#if $$slots.default}
    <slot />
  {:else}
    {label}
  {/if}
</span>
