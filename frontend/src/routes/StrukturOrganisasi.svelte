<script lang="ts">
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import { getStrukturOrganisasi } from '../lib/content';
  import { User, Users } from '../lib/components/icons';

  /**
   * Struktur Organisasi - vertical org chart with five tiers plus Staf:
   *   1. Kepala Desa  2. Sekretaris  3. Kasi (3)  4. Kaur (2)  5. Kadus (19)  6. Staf
   * Each entry carries a no_sk (Surat Keputusan) reference where available.
   */
  const data = getStrukturOrganisasi() as any;

  interface OrangPerangkat {
    nama: string | null;
    no_sk: string | null;
  }

  const kepalaDesa = (data?.kepala_desa ?? {}) as { jabatan?: string } & OrangPerangkat;
  const sekretaris = (data?.sekretaris_desa ?? {}) as { jabatan?: string } & OrangPerangkat;
  const kasi: Array<{ jabatan: string } & OrangPerangkat> = data?.kasi ?? [];
  const kaur: Array<{ jabatan: string } & OrangPerangkat> = data?.kaur ?? [];
  const kadus: Array<{ pedukuhan: string } & OrangPerangkat> = data?.kadus ?? [];
  const staf: OrangPerangkat[] = data?.staf ?? [];

  const isKosong = (nama: string | null | undefined): boolean =>
    !nama || nama.trim() === '' || nama.trim().toUpperCase() === 'TBD';

  const renderNama = (nama: string | null | undefined): string =>
    isKosong(nama) ? 'Kosong' : (nama as string);

  const vline = 'mx-auto h-8 w-px bg-krem-300';
</script>

<PageHeader
  eyebrow="Pemerintahan Desa"
  title="Struktur Organisasi"
  description="Susunan perangkat Desa Gerbosari beserta nomor Surat Keputusan."
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
      <div class="mt-1.5 font-serif text-xl font-semibold {isKosong(kepalaDesa.nama) ? 'italic text-arang-700/60' : 'text-arang-900'}">
        {renderNama(kepalaDesa.nama)}
      </div>
      {#if kepalaDesa.no_sk}
        <div class="mt-2 font-mono text-xs text-arang-700/70">SK {kepalaDesa.no_sk}</div>
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
      <div class="mt-1.5 font-serif text-lg font-medium {isKosong(sekretaris.nama) ? 'italic text-arang-700/60' : 'text-arang-900'}">
        {renderNama(sekretaris.nama)}
      </div>
      {#if sekretaris.no_sk}
        <div class="mt-1.5 font-mono text-xs text-arang-700/70">SK {sekretaris.no_sk}</div>
      {/if}
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
            <div class="mt-1.5 font-serif text-base {isKosong(item.nama) ? 'italic text-arang-700/60' : 'font-medium text-arang-900'}">
              {renderNama(item.nama)}
            </div>
            {#if item.no_sk}
              <div class="mt-1.5 font-mono text-xs text-arang-700/70">SK {item.no_sk}</div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <!-- ============ Divider: Kaur ============ -->
  {#if kaur.length > 0}
    <div class="mt-12 mb-6 flex items-center gap-4" aria-hidden="true">
      <div class="h-px flex-1 bg-krem-200"></div>
      <span class="text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/60">
        Kepala Urusan
      </span>
      <div class="h-px flex-1 bg-krem-200"></div>
    </div>

    <!-- ============ TIER 4: Kaur (2 cards) ============ -->
    <section aria-labelledby="kaur-title">
      <h3 id="kaur-title" class="sr-only">Kepala Urusan</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        {#each kaur as item}
          <article class="rounded-md border border-krem-200 bg-krem-50 p-5">
            <div class="text-[10px] font-semibold uppercase tracking-[0.16em] text-menoreh-700">
              {item.jabatan}
            </div>
            <div class="mt-1.5 font-serif text-base {isKosong(item.nama) ? 'italic text-arang-700/60' : 'font-medium text-arang-900'}">
              {renderNama(item.nama)}
            </div>
            {#if item.no_sk}
              <div class="mt-1.5 font-mono text-xs text-arang-700/70">SK {item.no_sk}</div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <!-- ============ Divider: Kadus ============ -->
  {#if kadus.length > 0}
    <div class="mt-14 mb-6 flex items-center gap-4">
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
            <div class="mt-2 border-t border-krem-200 pt-2 text-sm {isKosong(item.nama) ? 'italic text-arang-700/60' : 'text-arang-800'}">
              {renderNama(item.nama)}
            </div>
            {#if item.no_sk}
              <div class="mt-1 font-mono text-[11px] text-arang-700/60">SK {item.no_sk}</div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}

  <!-- ============ Divider: Staf ============ -->
  {#if staf.length > 0}
    <div class="mt-14 mb-6 flex items-center gap-4" aria-hidden="true">
      <div class="h-px flex-1 bg-krem-200"></div>
      <span class="text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/60">
        Staf
      </span>
      <div class="h-px flex-1 bg-krem-200"></div>
    </div>

    <!-- ============ TIER 6: Staf ============ -->
    <section aria-labelledby="staf-title">
      <h3 id="staf-title" class="sr-only">Staf</h3>
      <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
        {#each staf as item}
          <article class="rounded-md border border-krem-200 bg-krem-50 p-4">
            <div class="text-[10px] font-semibold uppercase tracking-[0.16em] text-arang-700/70">
              Staf
            </div>
            <div class="mt-1.5 font-serif text-base {isKosong(item.nama) ? 'italic text-arang-700/60' : 'font-medium text-arang-900'}">
              {renderNama(item.nama)}
            </div>
            {#if item.no_sk}
              <div class="mt-1.5 font-mono text-xs text-arang-700/70">SK {item.no_sk}</div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}

</section>
