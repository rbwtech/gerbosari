<script lang="ts">
  import { getStrukturOrganisasi } from '../lib/content';
  import { User, Users, Mountain } from '../lib/components/icons';

  /**
   * Struktur Organisasi - government-style organogram with batik-inspired warmth.
   *
   * Composition: hero band -> Kepala Desa apex -> connector -> Sekretaris ->
   * divider "Pelaksana Teknis" -> grouped Kaur (menoreh accent) + Kasi (terakota
   * accent) -> divider "Kepala Pedukuhan" -> 19-card grid (Sumbo rendered
   * "Belum terisi") -> divider "Staf" -> 3 compact staff cards.
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

  /** True when a name slot is empty / TBD / explicitly null. */
  const isKosong = (nama: string | null | undefined): boolean =>
    !nama || nama.trim() === '' || nama.trim().toUpperCase() === 'TBD';

  /** Inline tier-label utility. Mirrors the foundation `.eyebrow-village` token. */
  const eyebrow =
    'inline-block font-sans text-[10px] font-semibold uppercase tracking-[0.22em] text-terakota-700';
</script>

<!-- ============ HERO BAND ============ -->
<header
  class="relative overflow-hidden border-b border-krem-200 bg-tanah-paper"
  style="background-color: #faf7f2; background-image: radial-gradient(ellipse 60% 50% at 18% 28%, #fdf5f0 0%, transparent 62%), radial-gradient(ellipse 55% 45% at 82% 72%, #f5ebe0 0%, transparent 60%);"
>
  <div class="container-page py-14 md:py-20">
    <div class="flex flex-col items-center text-center">
      <!-- Village crest: hill + sun, terakota stroke -->
      <svg
        class="h-12 w-12 text-terakota-600"
        viewBox="0 0 48 48"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="24" cy="16" r="5" />
        <path d="M3 38 L16 22 L26 32 L34 24 L45 38 Z" fill="#fdf5f0" />
        <path d="M3 38 L16 22 L26 32 L34 24 L45 38" />
        <path d="M1 42 H47" stroke="#3e7242" />
      </svg>

      <p class="mt-5 {eyebrow}">Pemerintah Desa Gerbosari</p>

      <h1
        class="mt-3 font-serif text-4xl md:text-5xl font-semibold text-arang-900 text-balance"
        style="letter-spacing: -0.02em; line-height: 1.1;"
      >
        Struktur Organisasi
      </h1>

      <p class="mt-3 font-serif italic text-base md:text-lg text-tanah-700">
        Sejahtera Mandiri &mdash; Desa Wisata Berbasis Budaya dan Ekonomi Kreatif
      </p>

      <p class="mt-5 max-w-2xl text-sm md:text-base text-arang-700 leading-relaxed">
        Susunan perangkat Desa Gerbosari beserta jabatan dan nomor Surat
        Keputusan, dari Kepala Desa hingga Kepala Pedukuhan di 19 dusun.
      </p>
    </div>
  </div>
</header>

<div class="container-page py-12 md:py-16">
  <!-- ============ TIER 1: Kepala Desa apex ============ -->
  <section aria-labelledby="tier-kepala-desa">
    <h2 id="tier-kepala-desa" class="sr-only">Kepala Desa</h2>
    <div class="flex justify-center">
      <article
        class="w-full max-w-lg rounded-lg border border-terakota-500/60 bg-krem-50 p-5 md:p-6"
      >
        <div class="flex items-center gap-5">
          <!-- Photo placeholder block -->
          <div
            class="flex h-20 w-20 shrink-0 items-center justify-center rounded-md bg-krem-200 text-tanah-500 md:h-24 md:w-24"
            aria-hidden="true"
          >
            <User class="h-9 w-9" strokeWidth={1.5} />
          </div>

          <div class="min-w-0 flex-1">
            <span class={eyebrow}>{kepalaDesa.jabatan ?? 'Kepala Desa'}</span>
            <div
              class="mt-1 font-serif text-xl md:text-2xl font-semibold {isKosong(
                kepalaDesa.nama
              )
                ? 'italic text-arang-500'
                : 'text-arang-900'}"
              style="letter-spacing: -0.01em;"
            >
              {isKosong(kepalaDesa.nama) ? 'Belum terisi' : kepalaDesa.nama}
            </div>
            {#if kepalaDesa.no_sk}
              <div class="mt-2 font-mono text-[11px] tracking-tight text-arang-500">
                SK {kepalaDesa.no_sk}
              </div>
            {/if}
          </div>
        </div>
      </article>
    </div>
  </section>

  <!-- Connector: Kepala Desa -> Sekretaris (with diamond accent at midpoint) -->
  <div class="relative mx-auto my-1 h-10 w-px bg-menoreh-300" aria-hidden="true">
    <span
      class="absolute left-1/2 top-1/2 h-2 w-2 -translate-x-1/2 -translate-y-1/2 rotate-45 bg-terakota-500"
    ></span>
  </div>

  <!-- ============ TIER 2: Sekretaris Desa ============ -->
  <section aria-labelledby="tier-sekretaris">
    <h2 id="tier-sekretaris" class="sr-only">Sekretaris Desa</h2>
    <div class="flex justify-center">
      <article
        class="w-full max-w-md rounded-md border border-krem-300 border-l-2 border-l-terakota-500 bg-krem-50 p-4 md:p-5"
      >
        <div class="flex items-center gap-4">
          <div
            class="flex h-14 w-14 shrink-0 items-center justify-center rounded bg-krem-200 text-tanah-500"
            aria-hidden="true"
          >
            <User class="h-6 w-6" strokeWidth={1.5} />
          </div>
          <div class="min-w-0 flex-1">
            <span class={eyebrow}>{sekretaris.jabatan ?? 'Sekretaris Desa'}</span>
            <div
              class="mt-0.5 font-serif text-lg font-medium {isKosong(sekretaris.nama)
                ? 'italic text-arang-500'
                : 'text-arang-900'}"
            >
              {isKosong(sekretaris.nama) ? 'Belum terisi' : sekretaris.nama}
            </div>
            {#if sekretaris.no_sk}
              <div class="mt-1 font-mono text-[11px] tracking-tight text-arang-500">
                SK {sekretaris.no_sk}
              </div>
            {/if}
          </div>
        </div>
      </article>
    </div>
  </section>

  <!-- ============ Divider: Pelaksana Teknis ============ -->
  <div class="mt-12 mb-8 flex items-center gap-4">
    <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
    <span class={eyebrow}>Pelaksana Teknis</span>
    <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
  </div>

  <!-- ============ TIER 3+4: Kaur & Kasi grouped ============ -->
  <section aria-labelledby="tier-pelaksana">
    <h2 id="tier-pelaksana" class="sr-only">Pelaksana Teknis</h2>
    <div class="grid grid-cols-1 gap-8 md:grid-cols-5 md:gap-6">
      <!-- Kaur column (2 of 5) -->
      <div class="md:col-span-2">
        <div class="mb-3 flex items-center gap-2">
          <span class="h-1.5 w-1.5 rounded-full bg-menoreh-500" aria-hidden="true"></span>
          <span
            class="font-sans text-[10px] font-semibold uppercase tracking-[0.22em] text-menoreh-700"
          >
            Kepala Urusan
          </span>
        </div>
        <div class="grid grid-cols-1 gap-3">
          {#each kaur as item}
            <article
              class="rounded-md border border-menoreh-200 bg-menoreh-50 p-4"
            >
              <div
                class="font-sans text-[10px] font-semibold uppercase tracking-[0.16em] text-menoreh-700"
              >
                {item.jabatan}
              </div>
              <div
                class="mt-1 font-serif text-base {isKosong(item.nama)
                  ? 'italic text-arang-500'
                  : 'font-medium text-arang-900'}"
              >
                {isKosong(item.nama) ? 'Belum terisi' : item.nama}
              </div>
              {#if item.no_sk}
                <div class="mt-2 font-mono text-[11px] tracking-tight text-arang-500">
                  SK {item.no_sk}
                </div>
              {/if}
            </article>
          {/each}
        </div>
      </div>

      <!-- Kasi column (3 of 5) -->
      <div class="md:col-span-3">
        <div class="mb-3 flex items-center gap-2">
          <span class="h-1.5 w-1.5 rounded-full bg-terakota-500" aria-hidden="true"></span>
          <span
            class="font-sans text-[10px] font-semibold uppercase tracking-[0.22em] text-terakota-700"
          >
            Kepala Seksi
          </span>
        </div>
        <div class="grid grid-cols-1 gap-3">
          {#each kasi as item}
            <article
              class="rounded-md border border-terakota-200 bg-terakota-50 p-4"
            >
              <div
                class="font-sans text-[10px] font-semibold uppercase tracking-[0.16em] text-terakota-700"
              >
                {item.jabatan}
              </div>
              <div
                class="mt-1 font-serif text-base {isKosong(item.nama)
                  ? 'italic text-arang-500'
                  : 'font-medium text-arang-900'}"
              >
                {isKosong(item.nama) ? 'Belum terisi' : item.nama}
              </div>
              {#if item.no_sk}
                <div class="mt-2 font-mono text-[11px] tracking-tight text-arang-500">
                  SK {item.no_sk}
                </div>
              {/if}
            </article>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <!-- ============ Divider: Kepala Pedukuhan ============ -->
  <div class="mt-14 mb-8 flex items-center gap-4">
    <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
    <span class="inline-flex items-center gap-2 {eyebrow}">
      <Users class="h-3.5 w-3.5" strokeWidth={2} />
      Kepala Pedukuhan &middot; {kadus.length}
    </span>
    <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
  </div>

  <!-- ============ TIER 5: Kadus (19) ============ -->
  <section aria-labelledby="tier-kadus">
    <h2 id="tier-kadus" class="sr-only">Kepala Pedukuhan</h2>
    <div
      class="grid grid-cols-1 gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4"
    >
      {#each kadus as item}
        <article
          class="rounded-md border border-krem-200 bg-krem-50 px-4 py-3.5 {isKosong(
            item.nama
          )
            ? 'opacity-80'
            : ''}"
          style={isKosong(item.nama) ? 'background-color: #f5ebe0;' : undefined}
        >
          <div class="flex items-center gap-2">
            <span
              class="h-1.5 w-1.5 shrink-0 rounded-full bg-terakota-500"
              aria-hidden="true"
            ></span>
            <span class="font-serif text-[15px] font-medium text-arang-900">
              {item.pedukuhan}
            </span>
          </div>
          <div
            class="mt-2 border-t border-krem-200 pt-2 text-sm {isKosong(item.nama)
              ? 'italic text-arang-500'
              : 'text-arang-800'}"
          >
            {isKosong(item.nama) ? 'Belum terisi' : item.nama}
          </div>
          {#if item.no_sk}
            <div class="mt-1 font-mono text-[10px] tracking-tight text-arang-500">
              SK {item.no_sk}
            </div>
          {/if}
        </article>
      {/each}
    </div>
  </section>

  <!-- ============ Divider: Staf ============ -->
  {#if staf.length > 0}
    <div class="mt-14 mb-8 flex items-center gap-4">
      <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
      <span class={eyebrow}>Staf &middot; {staf.length}</span>
      <div class="h-px flex-1 bg-krem-300" aria-hidden="true"></div>
    </div>

    <!-- ============ TIER 6: Staf (3) ============ -->
    <section aria-labelledby="tier-staf">
      <h2 id="tier-staf" class="sr-only">Staf</h2>
      <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
        {#each staf as item}
          <article
            class="rounded-md border border-krem-200 bg-krem-50 px-4 py-3"
          >
            <div
              class="font-serif text-[15px] {isKosong(item.nama)
                ? 'italic text-arang-500'
                : 'font-medium text-arang-900'}"
            >
              {isKosong(item.nama) ? 'Belum terisi' : item.nama}
            </div>
            {#if item.no_sk}
              <div class="mt-1 font-mono text-[10px] tracking-tight text-arang-500">
                SK {item.no_sk}
              </div>
            {/if}
          </article>
        {/each}
      </div>
    </section>
  {/if}
</div>
