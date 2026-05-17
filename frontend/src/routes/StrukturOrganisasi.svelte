<script lang="ts">
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import { getStrukturOrganisasi } from '../lib/content';
  import { User, Users } from '../lib/components/icons';

  /**
   * Struktur Organisasi - vertical org chart with five tiers:
   *   1. Kepala Desa  2. Sekretaris  3. Kasi (3)  4. Kaur (3)  5. Kadus (19)
   * Most names in the source JSON are "TBD" until confirmed by aparatur desa,
   * so the page must degrade gracefully and render TBD as a muted italic state.
   */
  const data = getStrukturOrganisasi() as any;

  const kepalaDesa = data?.kepala_desa ?? {};
  const sekretaris = data?.sekretaris_desa ?? {};
  const kasi: Array<{ jabatan: string; nama: string }> = data?.kasi ?? [];
  const kaur: Array<{ jabatan: string; nama: string }> = data?.kaur ?? [];
  const kadus: Array<{ pedukuhan: string; nama: string }> = data?.kadus ?? [];

  const isTbd = (nama: string | undefined | null): boolean =>
    !nama || nama.trim().toUpperCase() === 'TBD';

  const renderNama = (nama: string | undefined | null): string =>
    isTbd(nama) ? 'TBD' : (nama as string);

  // Connector classes — thin vertical and horizontal lines built with bg.
  const vline = 'mx-auto h-8 w-px bg-krem-300';
</script>

<PageHeader
  eyebrow="Pemerintahan Desa"
  title="Struktur Organisasi"
  description="Susunan perangkat Desa Gerbosari — Kepala Desa, Sekretaris, Kasi, Kaur, dan 19 Kepala Pedukuhan."
/>

<section class="container-page py-12 md:py-16" aria-labelledby="struktur-title">
  <h2 id="struktur-title" class="sr-only">Bagan struktur organisasi</h2>

  <!-- ============ TIER 1: Kepala Desa ============ -->
  <div class="flex justify-center">
    <article class="w-full max-w-sm rounded-lg border border-menoreh-500/30 bg-menoreh-50/70 p-6 text-center">
      <div class="mx-auto inline-flex h-12 w-12 items-center justify-center rounded-full bg-menoreh-500 text-krem-50">
        <User class="h-6 w-6" strokeWidth={1.75} />
      </div>
      <div class="mt-3 text-[10px] font-semibold uppercase tracking-[0.18em] text-menoreh-700">
        Kepala Desa
      </div>
      <div class="mt-1.5 font-serif text-xl font-semibold {isTbd(kepalaDesa?.nama) ? 'italic text-arang-700/60' : 'text-arang-900'}">
        {renderNama(kepalaDesa?.nama)}
      </div>
      {#if kepalaDesa?.periode_terakhir_tercatat}
        <div class="mt-2 text-xs text-arang-700/70">
          Periode terakhir tercatat: {kepalaDesa.periode_terakhir_tercatat}
        </div>
      {/if}
    </article>
  </div>

  <!-- ============ Connector ============ -->
  <div class={vline} aria-hidden="true"></div>

  <!-- ============ TIER 2: Sekretaris ============ -->
  <div class="flex justify-center">
    <article class="w-full max-w-xs rounded-md border border-krem-200 bg-krem-50 p-5 text-center">
      <div class="text-[10px] font-semibold uppercase tracking-[0.18em] text-terakota-600">
        Sekretaris Desa
      </div>
      <div class="mt-1.5 font-serif text-lg font-medium {isTbd(sekretaris?.nama) ? 'italic text-arang-700/60' : 'text-arang-900'}">
        {renderNama(sekretaris?.nama)}
      </div>
    </article>
  </div>

  <!-- ============ Connector ============ -->
  {#if kasi.length > 0}
    <div class={vline} aria-hidden="true"></div>
  {/if}

  <!-- ============ TIER 3: Kasi (3 cards) ============ -->
  {#if kasi.length > 0}
    <section aria-labelledby="kasi-title">
      <h3 id="kasi-title" class="sr-only">Kepala Seksi</h3>
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        {#each kasi as item}
          <article class="rounded-md border border-krem-200 bg-krem-50 p-5">
            <div class="text-[10px] font-semibold uppercase tracking-[0.16em] text-terakota-600">
              {item.jabatan}
            </div>
            <div class="mt-1.5 font-serif text-base {isTbd(item.nama) ? 'italic text-arang-700/60' : 'font-medium text-arang-900'}">
              {renderNama(item.nama)}
            </div>
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <!-- ============ Divider ============ -->
  {#if kaur.length > 0}
    <div class="mt-12 mb-12 flex items-center gap-4" aria-hidden="true">
      <div class="h-px flex-1 bg-krem-200"></div>
      <span class="text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/60">
        Kepala Urusan
      </span>
      <div class="h-px flex-1 bg-krem-200"></div>
    </div>

    <!-- ============ TIER 4: Kaur ============ -->
    <section aria-labelledby="kaur-title">
      <h3 id="kaur-title" class="sr-only">Kepala Urusan</h3>
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
        {#each kaur as item}
          <article class="rounded-md border border-krem-200 bg-krem-50 p-5">
            <div class="text-[10px] font-semibold uppercase tracking-[0.16em] text-menoreh-700">
              {item.jabatan}
            </div>
            <div class="mt-1.5 font-serif text-base {isTbd(item.nama) ? 'italic text-arang-700/60' : 'font-medium text-arang-900'}">
              {renderNama(item.nama)}
            </div>
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <!-- ============ Divider ============ -->
  {#if kadus.length > 0}
    <div class="mt-14 mb-10 flex items-center gap-4">
      <div class="h-px flex-1 bg-krem-200" aria-hidden="true"></div>
      <div class="inline-flex items-center gap-2 text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/70">
        <Users class="h-3.5 w-3.5" strokeWidth={2} />
        <span>19 Kepala Pedukuhan</span>
      </div>
      <div class="h-px flex-1 bg-krem-200" aria-hidden="true"></div>
    </div>

    <!-- ============ TIER 5: Kadus (19) ============ -->
    <section aria-labelledby="kadus-title">
      <h3 id="kadus-title" class="sr-only">Kepala Pedukuhan</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3">
        {#each kadus as item}
          <article class="rounded-md border border-krem-200 bg-krem-50 px-4 py-3.5">
            <div class="text-[10px] font-semibold uppercase tracking-[0.14em] text-arang-700/70">
              Pedukuhan
            </div>
            <div class="mt-0.5 font-serif text-sm font-medium text-arang-900">{item.pedukuhan}</div>
            <div class="mt-2 border-t border-krem-200 pt-2 text-sm {isTbd(item.nama) ? 'italic text-arang-700/60' : 'text-arang-800'}">
              {renderNama(item.nama)}
            </div>
          </article>
        {/each}
      </div>
    </section>
  {/if}

</section>
