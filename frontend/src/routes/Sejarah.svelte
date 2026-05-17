<script lang="ts">
  import { getSejarah } from '../lib/content';
  import { ChevronDown, BookOpen, Landmark } from '../lib/components/icons';

  /**
   * Sejarah Desa — long-form, magazine-style narrative. Reads the rich JSON
   * shape directly (the legacy types.ts is intentionally bypassed via `any`
   * to avoid coupling this page to an outdated interface).
   */
  const data = getSejarah() as any;

  const pengantar: string = data?.pengantar ?? '';
  const asalUsul = data?.asal_usul ?? {};
  const kalurahanAsal: Array<{ nama: string; lurah: string; pedukuhan: string[]; catatan?: string }> =
    asalUsul?.kalurahan_asal ?? [];
  const etimologi = data?.etimologi ?? {};
  const etimKomponen: Array<{ suku_kata: string; berasal_dari: string }> =
    etimologi?.komponen ?? [];
  const peristiwa = data?.peristiwa_1949 ?? null;
  const legenda: Array<{ nama: string; isi: string }> = data?.legenda ?? [];
  const perubahanNama: Array<{ periode: string; sebutan: string; kepala: string }> =
    data?.pemerintahan?.perubahan_nama ?? [];
  const kepemimpinan: Array<{ periode: string; pemimpin: string; jabatan: string; catatan?: string }> =
    data?.pemerintahan?.kepemimpinan ?? [];

  // First legenda opens by default; chevron toggles the rest.
  let openLegenda: number | null = 0;
  const toggleLegenda = (i: number) => {
    openLegenda = openLegenda === i ? null : i;
  };

  // Split narrative text into paragraphs for the long peristiwa narrative.
  const splitParagraphs = (txt: string): string[] =>
    (txt ?? '').split(/\n\n+/).filter((p) => p.trim().length > 0);
</script>

<!-- ============================================================ HERO ============================================================ -->
<section
  class="relative isolate overflow-hidden border-b border-arang-900/20"
  aria-labelledby="sejarah-title"
>
  <img
    src="/images/cover/gerbosari-panorama-menoreh.jpg"
    alt=""
    class="absolute inset-0 -z-20 h-full w-full object-cover"
    loading="eager"
    decoding="async"
  />
  <div
    class="absolute inset-0 -z-10 bg-gradient-to-b from-arang-900/75 via-arang-900/70 to-arang-900/85"
    aria-hidden="true"
  ></div>

  <div class="container-page pt-24 pb-20 md:pt-28 md:pb-24 text-krem-50">
    <p class="eyebrow !text-terakota-300">Sejarah &amp; Asal Usul</p>
    <h1
      id="sejarah-title"
      class="mt-4 font-serif text-4xl md:text-6xl font-semibold leading-[1.05] text-balance max-w-4xl text-krem-50"
    >
      Dari kisah tiga kalurahan, lahirlah Gerbosari.
    </h1>
    {#if pengantar}
      <p class="mt-6 max-w-2xl text-base md:text-lg text-krem-100/90 leading-relaxed">
        {pengantar}
      </p>
    {/if}
  </div>
</section>

<!-- ============================================================ ETIMOLOGI ============================================================ -->
{#if etimKomponen.length > 0}
  <section class="container-page py-16 md:py-20" aria-labelledby="etim-title">
    <div class="max-w-2xl">
      <p class="eyebrow">Etimologi</p>
      <h2
        id="etim-title"
        class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight"
      >
        GER &middot; BO &middot; SA &middot; RI
      </h2>
      {#if etimologi?.isi}
        <p class="mt-4 text-arang-700 leading-relaxed">{etimologi.isi}</p>
      {/if}
    </div>

    <dl class="mt-10 grid grid-cols-2 lg:grid-cols-4 gap-3">
      {#each etimKomponen as k, i}
        <div class="rounded-lg border border-krem-200 bg-krem-50 p-6">
          <div class="flex items-baseline gap-2">
            <span class="font-serif text-5xl md:text-6xl font-semibold text-terakota-600 tnum leading-none">
              {k.suku_kata}
            </span>
            <span class="font-serif text-sm text-arang-700/60 tnum">0{i + 1}</span>
          </div>
          <div class="mt-5 text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/70">
            Berasal dari
          </div>
          <dd class="mt-1 font-serif text-base text-arang-900">{k.berasal_dari}</dd>
        </div>
      {/each}
    </dl>
  </section>
{/if}

<!-- ============================================================ ASAL USUL ============================================================ -->
{#if asalUsul?.isi}
  <section
    class="border-y border-krem-200 bg-krem-100/40"
    aria-labelledby="asal-title"
  >
    <div class="container-page py-16 md:py-20">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
        <div class="lg:col-span-7">
          <p class="eyebrow">Asal Usul</p>
          <h2
            id="asal-title"
            class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance"
          >
            {asalUsul?.judul ?? 'Asal Usul Desa Gerbosari'}
          </h2>
          <div class="mt-6 max-w-prose font-serif text-lg leading-relaxed text-arang-800">
            {#each splitParagraphs(asalUsul.isi) as paragraf}
              <p class="mt-5 first:mt-0">{paragraf}</p>
            {/each}
          </div>
        </div>

        {#if kalurahanAsal.length > 0}
          <aside class="lg:col-span-5" aria-label="Tiga kalurahan asal">
            <div class="text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/70">
              Tiga Kalurahan Asal
            </div>
            <ol class="mt-3 space-y-3">
              {#each kalurahanAsal as kal}
                <li class="rounded-md border border-krem-200 bg-krem-50 p-5">
                  <div class="flex items-baseline justify-between gap-3">
                    <h3 class="font-serif text-base font-semibold text-arang-900">{kal.nama}</h3>
                    <span class="text-[11px] uppercase tracking-wider text-arang-700/70 tnum">
                      {kal.pedukuhan?.length ?? 0} pedukuhan
                    </span>
                  </div>
                  <p class="mt-1 text-sm text-arang-700">
                    <span class="text-arang-700/70">Lurah:</span> {kal.lurah}
                  </p>
                  {#if kal.pedukuhan?.length}
                    <p class="mt-2 text-xs text-arang-700/85 leading-relaxed">
                      {kal.pedukuhan.join(' · ')}
                    </p>
                  {/if}
                  {#if kal.catatan}
                    <p class="mt-2 text-xs italic text-terakota-700/90">{kal.catatan}</p>
                  {/if}
                </li>
              {/each}
            </ol>
          </aside>
        {/if}
      </div>
    </div>
  </section>
{/if}

<!-- ============================================================ PERISTIWA 1949 ============================================================ -->
{#if peristiwa}
  <section class="container-page py-16 md:py-24" aria-labelledby="peristiwa-title">
    <div class="max-w-prose">
      <p class="eyebrow">Memori Kolektif</p>
      <h2
        id="peristiwa-title"
        class="mt-3 font-serif text-3xl md:text-4xl italic font-medium text-arang-900 leading-tight text-balance"
      >
        {peristiwa?.judul ?? 'Peristiwa 7 Maret 1949'}
      </h2>
      {#if peristiwa?.narator}
        <p class="mt-3 text-sm text-arang-700/80">
          Diceritakan oleh <span class="text-arang-900">{peristiwa.narator}</span>.
        </p>
      {/if}
    </div>

    <!-- Single narratively-justified left-rule: this IS a pull-quote source. -->
    <div class="mt-10 max-w-prose">
      {#if peristiwa?.kutipan}
        <blockquote class="border-l-4 border-terakota-500 pl-6 py-2">
          <p class="font-serif text-2xl md:text-3xl italic font-medium text-arang-900 leading-snug text-balance">
            &ldquo;{peristiwa.kutipan}&rdquo;
          </p>
        </blockquote>
      {/if}

      <div class="mt-8 font-serif text-lg leading-relaxed text-arang-800">
        {#each splitParagraphs(peristiwa?.narasi ?? '') as paragraf}
          <p class="mt-5 first:mt-0">{paragraf}</p>
        {/each}
      </div>

      {#if peristiwa?.korban_tercatat?.length}
        <div class="mt-10 rounded-md border border-krem-200 bg-krem-50 p-6">
          <div class="text-[10px] font-semibold uppercase tracking-[0.18em] text-terakota-600">
            Korban Tercatat
          </div>
          <ul class="mt-3 grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-1.5 text-sm text-arang-700">
            {#each peristiwa.korban_tercatat as nama}
              <li class="flex gap-2"><span class="text-arang-700/40">&mdash;</span>{nama}</li>
            {/each}
          </ul>
        </div>
      {/if}

      {#if peristiwa?.penutup}
        <p class="mt-10 font-serif text-xl italic text-terakota-700 text-center">
          {peristiwa.penutup}
        </p>
      {/if}
    </div>
  </section>
{/if}

<!-- ============================================================ LEGENDA ============================================================ -->
{#if legenda.length > 0}
  <section
    class="border-t border-krem-200 bg-menoreh-50/50"
    aria-labelledby="legenda-title"
  >
    <div class="container-page py-16 md:py-20">
      <div class="max-w-2xl">
        <div class="flex items-center gap-2">
          <BookOpen class="h-4 w-4 text-menoreh-700" strokeWidth={2} />
          <p class="eyebrow">Tutur Pinisepuh</p>
        </div>
        <h2
          id="legenda-title"
          class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight"
        >
          Legenda Desa
        </h2>
        <p class="mt-4 text-arang-700 leading-relaxed">
          Cerita tutur yang masih diwariskan secara lisan oleh pinisepuh,
          mengaitkan tempat-tempat keramat dengan tokoh dan peristiwa masa lalu.
        </p>
      </div>

      <ul class="mt-10 divide-y divide-krem-200 border-y border-krem-200 bg-krem-50/60">
        {#each legenda as item, i}
          {@const isOpen = openLegenda === i}
          <li>
            <h3>
              <button
                type="button"
                class="flex w-full items-center justify-between gap-4 px-2 py-5 text-left transition-colors hover:bg-krem-50"
                aria-expanded={isOpen}
                on:click={() => toggleLegenda(i)}
              >
                <span class="flex items-baseline gap-4">
                  <span class="font-serif text-sm text-arang-700/60 tnum">
                    {String(i + 1).padStart(2, '0')}
                  </span>
                  <span class="font-serif text-lg md:text-xl font-medium text-arang-900">
                    {item.nama}
                  </span>
                </span>
                <ChevronDown
                  class="h-5 w-5 shrink-0 text-arang-700/70 transition-transform {isOpen ? 'rotate-180' : ''}"
                  strokeWidth={1.75}
                />
              </button>
            </h3>
            {#if isOpen}
              <div class="px-2 pb-6 pl-12">
                <div class="max-w-prose font-serif text-base leading-relaxed text-arang-800">
                  {#each splitParagraphs(item.isi) as paragraf}
                    <p class="mt-4 first:mt-0">{paragraf}</p>
                  {/each}
                </div>
              </div>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  </section>
{/if}

<!-- ============================================================ PEMERINTAHAN ============================================================ -->
{#if kepemimpinan.length > 0}
  <section class="container-page py-16 md:py-20" aria-labelledby="pemerintahan-title">
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
      <div class="lg:col-span-4">
        <div class="flex items-center gap-2">
          <Landmark class="h-4 w-4 text-menoreh-700" strokeWidth={2} />
          <p class="eyebrow">Riwayat Pemerintahan</p>
        </div>
        <h2
          id="pemerintahan-title"
          class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight"
        >
          Garis kepemimpinan desa.
        </h2>
        <p class="mt-4 text-arang-700 leading-relaxed">
          Sejak penggabungan 1947, sebutan jabatan kepala desa mengalami beberapa
          perubahan mengikuti regulasi nasional.
        </p>

        {#if perubahanNama.length > 0}
          <dl class="mt-8 space-y-3">
            {#each perubahanNama as pn}
              <div class="flex items-baseline gap-3 text-sm">
                <dt class="font-mono text-xs text-arang-700/70 tnum shrink-0 w-24">{pn.periode}</dt>
                <dd class="text-arang-700">
                  <span class="text-arang-900">{pn.sebutan}</span> &middot; {pn.kepala}
                </dd>
              </div>
            {/each}
          </dl>
        {/if}
      </div>

      <div class="lg:col-span-8">
        <ol class="relative">
          {#each kepemimpinan as p, i}
            <li class="grid grid-cols-[7rem_1fr] gap-6 py-5 {i !== kepemimpinan.length - 1 ? 'border-b border-krem-200' : ''}">
              <div class="font-mono text-xs text-arang-700/80 tnum pt-1">{p.periode}</div>
              <div>
                <div class="font-serif text-lg font-semibold text-arang-900">{p.pemimpin}</div>
                <div class="text-sm text-arang-700">{p.jabatan}</div>
                {#if p.catatan}
                  <div class="mt-1 text-xs italic text-arang-700/80">{p.catatan}</div>
                {/if}
              </div>
            </li>
          {/each}
        </ol>
      </div>
    </div>
  </section>
{/if}
