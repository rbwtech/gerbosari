<script lang="ts">
  import { getSejarah } from '../lib/content';
  import { ChevronDown, BookOpen, Landmark } from '../lib/components/icons';
  import SectionShell from '../lib/components/ui/SectionShell.svelte';

  /**
   * Sejarah Desa - long-form, magazine-style narrative. Reads the rich JSON
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
  class="relative isolate overflow-hidden border-b border-arang-900/20 flex items-center min-h-[55vh] md:min-h-[60vh]"
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
    class="absolute inset-0 -z-10 bg-gradient-to-b from-arang-900/90 via-arang-900/65 to-arang-900/90"
    aria-hidden="true"
  ></div>

  <div class="container-page py-12 md:py-16 text-white w-full">
    <p
      class="eyebrow !text-white break-words"
      style="text-shadow: 0 1px 3px rgba(0,0,0,0.6);"
    >
      Sejarah &amp; Asal Usul
    </p>
    <h1
      id="sejarah-title"
      class="mt-4 font-serif text-3xl md:text-6xl font-semibold leading-[1.05] text-balance max-w-4xl text-white break-words"
      style="text-shadow: 0 2px 6px rgba(0,0,0,0.45);"
    >
      Dari kisah tiga kalurahan, lahirlah Gerbosari.
    </h1>
    {#if pengantar}
      <p
        class="mt-6 max-w-2xl text-base md:text-lg text-white leading-relaxed text-pretty break-words"
        style="text-shadow: 0 1px 4px rgba(0,0,0,0.5);"
      >
        {pengantar}
      </p>
    {/if}
  </div>
</section>

<!-- ============================================================ ETIMOLOGI ============================================================ -->
<!-- Tanah biome: warm tinted paper so the big terakota suku-kata read like
     embossed wood-block letters on heritage parchment. -->
{#if etimKomponen.length > 0}
  <SectionShell variant="tanah" padding="md">
    <div aria-labelledby="etim-title">
      <div class="max-w-2xl">
        <p class="eyebrow">Etimologi</p>
        <h2
          id="etim-title"
          class="mt-3 font-serif text-2xl md:text-4xl font-semibold text-arang-900 leading-tight break-words"
        >
          GER &middot; BO &middot; SA &middot; RI
        </h2>
        {#if etimologi?.isi}
          <p class="mt-4 text-arang-700 leading-relaxed text-pretty break-words">{etimologi.isi}</p>
        {/if}
      </div>

      <!-- 2x2 grid on mobile so the four sukukata stay legible side by side;
           md+ goes back to the originally-designed 4-up row. -->
      <dl class="mt-10 grid grid-cols-2 md:grid-cols-4 gap-3">
        {#each etimKomponen as k, i}
          <div class="rounded-lg border border-tanah-200 bg-white p-5 md:p-6 min-w-0">
            <div class="flex items-baseline gap-2">
              <span class="font-serif text-5xl md:text-7xl font-semibold text-terakota-600 tnum leading-none break-words">
                {k.suku_kata}
              </span>
              <span class="font-serif text-sm text-arang-700/60 tnum">0{i + 1}</span>
            </div>
            <div class="mt-4 md:mt-5 text-[10px] font-semibold uppercase tracking-[0.18em] text-arang-700/70 break-words">
              Berasal dari
            </div>
            <dd class="mt-1 font-serif text-base text-arang-900 break-words">{k.berasal_dari}</dd>
          </div>
        {/each}
      </dl>
    </div>
  </SectionShell>
{/if}

<!-- ============================================================ ASAL USUL ============================================================ -->
<!-- Default paper bg for readable long-form narrative. -->
{#if asalUsul?.isi}
  <SectionShell variant="default" padding="lg">
    <div aria-labelledby="asal-title">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
        <div class="lg:col-span-7">
          <p class="eyebrow">Asal Usul</p>
          <h2
            id="asal-title"
            class="mt-3 font-serif text-2xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance break-words"
          >
            {asalUsul?.judul ?? 'Asal Usul Desa Gerbosari'}
          </h2>
          <div class="mt-6 max-w-prose font-serif text-base md:text-lg leading-relaxed text-arang-800">
            {#each splitParagraphs(asalUsul?.isi ?? '') as paragraf}
              <p class="mt-5 first:mt-0 text-pretty break-words">{paragraf}</p>
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
                <li class="rounded-md border border-krem-200 bg-white p-4 md:p-5 min-w-0">
                  <div class="flex flex-wrap items-baseline justify-between gap-x-3 gap-y-1">
                    <h3 class="font-serif text-base font-semibold text-arang-900 break-words">{kal.nama}</h3>
                    <span class="text-[11px] uppercase tracking-wider text-arang-700/70 tnum">
                      {kal.pedukuhan?.length ?? 0} pedukuhan
                    </span>
                  </div>
                  <p class="mt-1 text-sm text-arang-700 break-words">
                    <span class="text-arang-700/70">Lurah:</span> {kal.lurah}
                  </p>
                  {#if kal.pedukuhan?.length}
                    <p class="mt-2 text-xs text-arang-700/85 leading-relaxed break-words">
                      {kal.pedukuhan.join(' · ')}
                    </p>
                  {/if}
                  {#if kal.catatan}
                    <p class="mt-2 text-xs italic text-terakota-700/90 break-words">{kal.catatan}</p>
                  {/if}
                </li>
              {/each}
            </ol>
          </aside>
        {/if}
      </div>
    </div>
  </SectionShell>
{/if}

<!-- ============================================================ PERISTIWA 1949 ============================================================ -->
<!-- The dramatic visual moment of the page. Dark menoreh-deep biome - mourning,
     memorial - with a brighter terakota-400 pull-quote rule for contrast. -->
{#if peristiwa}
  <SectionShell variant="menoreh-deep" padding="lg">
    <div aria-labelledby="peristiwa-title">
      <div class="max-w-prose">
        <p class="text-xs font-semibold uppercase text-terakota-300" style="letter-spacing: 0.22em;">
          Memori Kolektif
        </p>
        <h2
          id="peristiwa-title"
          class="mt-3 font-serif text-2xl md:text-4xl italic font-medium text-krem-50 leading-tight text-balance break-words"
        >
          {peristiwa?.judul ?? 'Peristiwa 7 Maret 1949'}
        </h2>
        {#if peristiwa?.narator}
          <p class="mt-3 text-sm text-krem-100/70 break-words">
            Diceritakan oleh <span class="text-krem-50">{peristiwa.narator}</span>.
          </p>
        {/if}
      </div>

      <!-- Single narratively-justified left-rule: this IS a pull-quote source.
           Pull-quote scale steps down on mobile (xl) so it stays readable on
           360px rather than blowing out to 3xl/4xl headline-sized italics. -->
      <div class="mt-10 max-w-prose">
        {#if peristiwa?.kutipan}
          <blockquote class="border-l-4 border-terakota-400 pl-5 md:pl-6 py-2">
            <p class="font-serif text-xl md:text-2xl italic font-medium text-krem-50 leading-snug text-balance break-words">
              &ldquo;{peristiwa.kutipan}&rdquo;
            </p>
          </blockquote>
        {/if}

        <div class="mt-8 font-serif text-base md:text-lg leading-relaxed text-krem-100">
          {#each splitParagraphs(peristiwa?.narasi ?? '') as paragraf}
            <p class="mt-5 first:mt-0 text-pretty break-words">{paragraf}</p>
          {/each}
        </div>

        {#if peristiwa?.korban_tercatat?.length}
          <div class="mt-10 rounded-md border border-menoreh-700 bg-menoreh-800/60 p-5 md:p-6">
            <div class="text-[10px] font-semibold uppercase tracking-[0.18em] text-terakota-300">
              Korban Tercatat
            </div>
            <ul class="mt-3 grid grid-cols-1 sm:grid-cols-2 gap-x-6 gap-y-1.5 text-sm text-krem-100">
              {#each peristiwa.korban_tercatat as nama}
                <li class="flex gap-2 break-words"><span class="text-krem-100/40 shrink-0">-</span>{nama}</li>
              {/each}
            </ul>
          </div>
        {/if}

        {#if peristiwa?.penutup}
          <p class="mt-10 font-serif text-lg md:text-xl italic text-terakota-300 text-center text-balance break-words">
            {peristiwa.penutup}
          </p>
        {/if}
      </div>
    </div>
  </SectionShell>
{/if}

<!-- ============================================================ LEGENDA ============================================================ -->
<!-- Batik biome: subtle terakota motif behind the folklore accordion. -->
{#if legenda.length > 0}
  <SectionShell variant="batik" padding="lg">
    <div aria-labelledby="legenda-title">
      <div class="max-w-2xl">
        <div class="flex items-center gap-2">
          <BookOpen class="h-4 w-4 text-menoreh-700" strokeWidth={2} />
          <p class="eyebrow">Tutur Pinisepuh</p>
        </div>
        <h2
          id="legenda-title"
          class="mt-3 font-serif text-2xl md:text-4xl font-semibold text-arang-900 leading-tight break-words"
        >
          Legenda Desa
        </h2>
        <p class="mt-4 text-arang-700 leading-relaxed text-pretty break-words">
          Cerita tutur yang masih diwariskan secara lisan oleh pinisepuh,
          mengaitkan tempat-tempat keramat dengan tokoh dan peristiwa masa lalu.
        </p>
      </div>

      <!-- Accordion items are individual white cards on the batik motif so each
           legenda reads as its own folio rather than a flat divided list.
           min-h-14 keeps the tap surface comfortable per HIG. -->
      <ul class="mt-10 space-y-2">
        {#each legenda as item, i}
          {@const isOpen = openLegenda === i}
          <li class="rounded-md border border-krem-300 bg-white transition-colors hover:border-terakota-400 min-w-0">
            <h3>
              <button
                type="button"
                class="flex w-full items-center justify-between gap-3 sm:gap-4 min-h-14 px-4 sm:px-5 py-4 text-left"
                aria-expanded={isOpen}
                on:click={() => toggleLegenda(i)}
              >
                <span class="flex items-baseline gap-3 sm:gap-4 min-w-0">
                  <span class="font-serif text-sm text-arang-700/60 tnum shrink-0">
                    {String(i + 1).padStart(2, '0')}
                  </span>
                  <span class="font-serif text-base md:text-xl font-medium text-arang-900 break-words">
                    {item?.nama ?? ''}
                  </span>
                </span>
                <ChevronDown
                  class="h-5 w-5 shrink-0 text-arang-700/70 transition-transform duration-200 ease-out {isOpen ? 'rotate-180' : ''}"
                  strokeWidth={1.75}
                />
              </button>
            </h3>
            {#if isOpen}
              <!-- On mobile, drop the pl-16 indent so the body uses the full
                   card width; restore on sm+ where the indent reads as a
                   typographic flourish next to the leading numeral. -->
              <div class="px-4 sm:px-5 pb-6 sm:pl-16">
                <div class="max-w-prose font-serif text-base leading-relaxed text-arang-800">
                  {#each splitParagraphs(item?.isi ?? '') as paragraf}
                    <p class="mt-4 first:mt-0 text-pretty break-words">{paragraf}</p>
                  {/each}
                </div>
              </div>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  </SectionShell>
{/if}

<!-- ============================================================ PEMERINTAHAN ============================================================ -->
{#if kepemimpinan.length > 0}
  <SectionShell variant="default" padding="md">
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-12" aria-labelledby="pemerintahan-title">
      <div class="lg:col-span-4">
        <div class="flex items-center gap-2">
          <Landmark class="h-4 w-4 text-menoreh-700" strokeWidth={2} />
          <p class="eyebrow">Riwayat Pemerintahan</p>
        </div>
        <h2
          id="pemerintahan-title"
          class="mt-3 font-serif text-2xl md:text-4xl font-semibold text-arang-900 leading-tight break-words"
        >
          Garis kepemimpinan desa.
        </h2>
        <p class="mt-4 text-arang-700 leading-relaxed text-pretty break-words">
          Sejak penggabungan 1947, sebutan jabatan kepala desa mengalami beberapa
          perubahan mengikuti regulasi nasional.
        </p>

        {#if perubahanNama.length > 0}
          <dl class="mt-8 space-y-3">
            {#each perubahanNama as pn}
              <div class="flex flex-col sm:flex-row sm:items-baseline gap-1 sm:gap-3 text-sm">
                <!-- Tabular nums + tanah-700 anchor the timeline columns as a
                     printed register, not a generic table row. Stacks vertical
                     on tiny viewports to keep periode + sebutan readable. -->
                <dt class="font-mono text-xs md:text-sm text-tanah-700 tnum shrink-0 sm:w-24">{pn.periode}</dt>
                <dd class="text-arang-700 break-words">
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
            <!-- Single-column vertical stack on mobile: 7rem fixed-column grid
                 was forcing the names to wrap at awkward points on 360px.
                 sm+ restores the two-column periode/details layout. -->
            <li class="flex flex-col sm:grid sm:grid-cols-[7rem_1fr] gap-1 sm:gap-6 py-5 {i !== kepemimpinan.length - 1 ? 'border-b border-krem-200' : ''}">
              <div class="font-mono text-xs md:text-sm text-tanah-700 tnum sm:pt-1">{p.periode}</div>
              <div class="min-w-0">
                <div class="font-serif text-lg font-semibold text-arang-900 break-words">{p.pemimpin}</div>
                <div class="text-sm text-arang-700 break-words">{p.jabatan}</div>
                {#if p.catatan}
                  <div class="mt-1 text-xs italic text-arang-700/80 break-words">{p.catatan}</div>
                {/if}
              </div>
            </li>
          {/each}
        </ol>
      </div>
    </div>
  </SectionShell>
{/if}
