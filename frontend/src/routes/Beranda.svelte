<script lang="ts">
  import { link } from '../lib/router';
  import { getBeranda, getPetaWilayah } from '../lib/content';
  import Button from '../lib/components/ui/Button.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import {
    ArrowRight,
    Mountain,
    Compass,
    Newspaper,
    ImageIcon,
    Landmark,
    Coffee,
    TreePine,
    Sparkles,
    MapPin,
    Phone
  } from '../lib/components/icons';

  /**
   * Beranda - magazine-style cover page. Pulls hero, stats, zona, and key
   * highlights from content/pages/beranda.json + peta-wilayah.json. All data
   * is read defensively because the source JSON can drift ahead of the typed
   * shape during the content pipeline.
   */
  const beranda = getBeranda() as any;
  const peta = getPetaWilayah() as any;

  const hero = beranda?.hero ?? {};
  const alamat = beranda?.alamat ?? {};
  const quickStats: Array<{ label: string; nilai: string; satuan: string }> =
    beranda?.quick_stats ?? [];
  const highlights: Array<{ judul: string; deskripsi: string }> =
    beranda?.highlights ?? [];
  const zona: Array<{ nomor: number; tema: string; lokasi?: string[]; lokasi_budidaya_ikan?: string[]; lokasi_empon_empon?: string[] }> =
    peta?.zona ?? [];

  // Curate the 4 stats the spec asked for as the headline grid.
  const headlineStatLabels = ['Luas Wilayah', 'Jumlah Penduduk', 'Pedukuhan', 'Ketinggian'];
  const headlineStats = headlineStatLabels
    .map((label) => quickStats.find((s) => s.label === label))
    .filter((s): s is { label: string; nilai: string; satuan: string } => Boolean(s));

  // Map zona nomor -> icon. Zone 4 uses TreePine (kombinasi perikanan + empon-empon).
  const zonaIcon = (n: number) => {
    if (n === 1) return Coffee;
    if (n === 2) return Mountain;
    if (n === 3) return Sparkles;
    return TreePine;
  };

  const zonaJudul = (n: number) => {
    if (n === 1) return 'Kopi & Teh Suroloyo';
    if (n === 2) return 'Durian & Kambing PE';
    if (n === 3) return 'Krisan, Batik & Budaya';
    return 'Perikanan & Empon-empon';
  };

  // For Zona 4, JSON stores two arrays. Merge them for display.
  const zonaLokasi = (z: { lokasi?: string[]; lokasi_budidaya_ikan?: string[]; lokasi_empon_empon?: string[] }): string[] => {
    if (z.lokasi) return z.lokasi;
    return [...(z.lokasi_budidaya_ikan ?? []), ...(z.lokasi_empon_empon ?? [])];
  };

  const links = [
    { href: '/sejarah', title: 'Sejarah Desa', desc: 'Asal-usul, etimologi GER-BO-SA-RI, dan peristiwa 1949.', Icon: Landmark },
    { href: '/visi-misi', title: 'Visi & Misi', desc: 'Arah pembangunan desa dan kabupaten Kulon Progo.', Icon: Compass },
    { href: '/galeri', title: 'Galeri Foto', desc: 'Dokumentasi kegiatan, kerajinan, dan agrowisata.', Icon: ImageIcon },
    { href: '/berita', title: 'Berita & Agenda', desc: 'Kabar pemerintahan desa dan jadwal kegiatan.', Icon: Newspaper }
  ];
</script>

<!-- ============================================================ HERO ============================================================ -->
<section
  class="relative isolate overflow-hidden border-b border-arang-900/20"
  aria-labelledby="hero-title"
>
  <img
    src="/images/cover/gerbosari-panorama-sawah-menoreh.jpg"
    alt=""
    class="absolute inset-0 -z-20 h-full w-full object-cover"
    loading="eager"
    decoding="async"
  />
  <!-- Tonal scrim: deep arang at bottom for legibility, subtle menoreh tint up top. -->
  <div
    class="absolute inset-0 -z-10 bg-gradient-to-b from-arang-900/75 via-arang-900/70 to-arang-900/85"
    aria-hidden="true"
  ></div>

  <div class="container-page pt-24 pb-20 md:pt-32 md:pb-28 text-krem-50">
    <p class="eyebrow !text-terakota-300">
      Kabupaten Kulon Progo &middot; Daerah Istimewa Yogyakarta
    </p>
    <h1
      id="hero-title"
      class="mt-4 font-serif text-5xl md:text-7xl font-semibold leading-[1.05] text-balance max-w-4xl text-krem-50"
    >
      {hero?.judul ?? 'Desa Gerbosari'}
    </h1>
    <p class="mt-5 font-serif text-xl md:text-2xl italic text-krem-100/95 max-w-3xl text-balance">
      {hero?.subjudul ?? 'Sejahtera Mandiri'} - {hero?.tagline ?? 'Desa Wisata Berbasis Budaya dan Ekonomi Kreatif'}
    </p>

    <!-- Honor mention pill row -->
    <ul class="mt-8 flex flex-wrap gap-2">
      <li>
        <span class="inline-flex items-center gap-2 rounded-full border border-krem-50/25 bg-krem-50/10 px-3 py-1 text-xs font-medium text-krem-50 backdrop-blur-sm">
          <Sparkles class="h-3.5 w-3.5" strokeWidth={2} />
          Juara II Lomba Desa Tingkat DIY 2018
        </span>
      </li>
      <li>
        <span class="inline-flex items-center gap-2 rounded-full border border-krem-50/25 bg-krem-50/10 px-3 py-1 text-xs font-medium text-krem-50 backdrop-blur-sm">
          <Mountain class="h-3.5 w-3.5" strokeWidth={2} />
          Lereng Perbukitan Menoreh
        </span>
      </li>
      <li>
        <span class="inline-flex items-center gap-2 rounded-full border border-krem-50/25 bg-krem-50/10 px-3 py-1 text-xs font-medium text-krem-50 backdrop-blur-sm">
          <Coffee class="h-3.5 w-3.5" strokeWidth={2} />
          Agrowisata Kopi &amp; Teh
        </span>
      </li>
    </ul>

    <div class="mt-10 flex flex-wrap gap-3">
      <a
        href="/sejarah"
        use:link
        class="inline-flex h-11 items-center justify-center gap-2 rounded-md bg-terakota-500 px-5 text-sm font-medium text-krem-50 transition-colors hover:bg-terakota-600"
      >
        Telusuri Sejarah
        <ArrowRight class="h-4 w-4" strokeWidth={2} />
      </a>
      <a
        href="/peta-wilayah"
        use:link
        class="inline-flex h-11 items-center justify-center gap-2 rounded-md border border-krem-50/40 bg-krem-50/5 px-5 text-sm font-medium text-krem-50 transition-colors hover:bg-krem-50/10"
      >
        Lihat Peta Wilayah
      </a>
    </div>
  </div>
</section>

<!-- ============================================================ INTRO + STATS ============================================================ -->
<section
  class="container-page py-16 md:py-20"
  aria-labelledby="intro-title"
>
  <div class="grid grid-cols-1 lg:grid-cols-12 gap-12 lg:gap-16">
    <div class="lg:col-span-7">
      <p class="eyebrow">Sekilas Desa</p>
      <h2
        id="intro-title"
        class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance"
      >
        Di lereng Menoreh, sebuah desa lahir dari tiga kalurahan.
      </h2>
      <div class="prose-village mt-6">
        <p>{hero?.deskripsi_singkat ?? ''}</p>
        <p>
          Nama <em>Gerbosari</em> sendiri tersusun dari potongan suku kata tiga kalurahan
          yang dileburkan tahun 1947 - Mengger<strong>mal</strong>ang, Kemiri<strong>om</strong>bo, dan <strong>Sa</strong>migaluh
          - ditambah satu suku kata penutup, <strong>RI</strong>.
        </p>
      </div>
    </div>

    <aside class="lg:col-span-5" aria-label="Statistik desa">
      <div class="grid grid-cols-2 divide-x divide-y divide-krem-200 border border-krem-200 rounded-lg overflow-hidden bg-krem-50">
        {#each headlineStats as s}
          <div class="p-5 md:p-6 min-w-0">
            <div class="text-[11px] font-medium uppercase tracking-[0.14em] text-arang-700/70">{s.label}</div>
            <div class="mt-2 flex flex-col">
              <span class="font-serif text-xl md:text-2xl font-semibold text-arang-900 tnum leading-tight break-words">{s.nilai}</span>
              <span class="mt-0.5 text-[11px] text-arang-700">{s.satuan}</span>
            </div>
          </div>
        {/each}
      </div>
    </aside>
  </div>
</section>

<!-- ============================================================ EMPAT KAWASAN ============================================================ -->
<section
  class="border-y border-krem-200 bg-menoreh-50/60"
  aria-labelledby="zona-title"
>
  <div class="container-page py-16 md:py-20">
    <div class="max-w-2xl">
      <p class="eyebrow">Pembangunan Terpadu</p>
      <h2
        id="zona-title"
        class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance"
      >
        Empat Kawasan Ekonomi
      </h2>
      <p class="mt-4 text-arang-700 leading-relaxed">
        Sembilan belas pedukuhan disusun ke dalam empat zona tematik - dari kopi-teh
        Suroloyo hingga perikanan dan empon-empon di lembah timur.
      </p>
    </div>

    {#if zona.length > 0}
      <div class="mt-10 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {#each zona as z}
          {@const Icon = zonaIcon(z.nomor)}
          {@const lokasi = zonaLokasi(z)}
          <Card padded={false} class="flex flex-col h-full">
            <div class="p-6 flex-1">
              <div class="flex items-center justify-between">
                <span class="inline-flex h-10 w-10 items-center justify-center rounded-md bg-menoreh-50 text-menoreh-700">
                  <svelte:component this={Icon} class="h-5 w-5" strokeWidth={1.75} />
                </span>
                <span class="font-serif text-3xl font-semibold text-terakota-500/80 tnum">
                  0{z.nomor}
                </span>
              </div>
              <h3 class="mt-5 font-serif text-lg font-semibold text-arang-900">
                Zona {z.nomor} &middot; {zonaJudul(z.nomor)}
              </h3>
              <p class="mt-2 text-sm text-arang-700 leading-relaxed">{z.tema}</p>
            </div>
            {#if lokasi.length > 0}
              <div class="border-t border-krem-200 p-5">
                <div class="text-[10px] font-semibold uppercase tracking-[0.16em] text-arang-700/70">
                  Pedukuhan
                </div>
                <div class="mt-2 flex flex-wrap gap-1.5">
                  {#each lokasi as p}
                    <span class="inline-flex items-center rounded-sm border border-krem-200 bg-krem-100 px-1.5 py-0.5 text-[11px] font-medium uppercase tracking-wide text-arang-700">
                      {p}
                    </span>
                  {/each}
                </div>
              </div>
            {/if}
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</section>

<!-- ============================================================ JELAJAHI ============================================================ -->
<section class="container-page py-16 md:py-20" aria-labelledby="jelajahi-title">
  <div class="flex items-end justify-between gap-4 mb-10">
    <div>
      <p class="eyebrow">Navigasi</p>
      <h2
        id="jelajahi-title"
        class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900"
      >
        Jelajahi Desa
      </h2>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
    {#each links as item}
      <a
        href={item.href}
        use:link
        class="group flex items-start gap-5 rounded-lg border border-krem-200 bg-krem-50 p-6 transition-colors hover:border-menoreh-500/40 hover:bg-menoreh-50/40"
      >
        <span class="shrink-0 inline-flex h-11 w-11 items-center justify-center rounded-md bg-menoreh-50 text-menoreh-700">
          <svelte:component this={item.Icon} class="h-5 w-5" strokeWidth={1.75} />
        </span>
        <div class="flex-1 min-w-0">
          <div class="flex items-center justify-between gap-3">
            <h3 class="font-serif text-lg font-semibold text-arang-900">{item.title}</h3>
            <ArrowRight class="h-4 w-4 text-arang-700/40 transition-colors group-hover:text-menoreh-600" strokeWidth={2} />
          </div>
          <p class="mt-1 text-sm text-arang-700 leading-relaxed">{item.desc}</p>
        </div>
      </a>
    {/each}
  </div>
</section>

<!-- ============================================================ SEKILAS WISATA: SUROLOYO ============================================================ -->
<section
  class="border-t border-krem-200 bg-krem-100/40"
  aria-labelledby="suroloyo-title"
>
  <div class="container-page py-16 md:py-24">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-10 lg:gap-16 items-center">
      <figure class="order-2 lg:order-1">
        <img
          src="/images/gallery/gerbosari-puncak-suroloyo.png"
          alt="Gardu pandang Puncak Suroloyo, titik tertinggi perbukitan Menoreh"
          class="w-full aspect-[4/3] object-cover rounded-lg border border-krem-200"
          loading="lazy"
          decoding="async"
        />
        <figcaption class="mt-3 text-xs uppercase tracking-[0.16em] text-arang-700/70">
          Pemandangan Puncak Suroloyo &middot; Pedukuhan Keceme
        </figcaption>
      </figure>

      <div class="order-1 lg:order-2">
        <p class="eyebrow">Sekilas Wisata</p>
        <h2
          id="suroloyo-title"
          class="mt-3 font-serif text-3xl md:text-4xl font-semibold text-arang-900 leading-tight text-balance"
        >
          Puncak Suroloyo - kahyangan para dewa di ketinggian seribu meter.
        </h2>
        <p class="mt-5 text-arang-700 leading-relaxed">
          Titik tertinggi perbukitan Menoreh, sekitar 1.000 m dpl, dipercaya sebagai
          tempat bertapa Raden Mas Rangsang - Sultan Agung muda - setelah didatangi
          Sunan Kalijaga dalam mimpi. Dari puncaknya tampak siluet Borobudur, Merapi-Merbabu,
          Sumbing-Sundoro, hingga Sungai Progo di kejauhan.
        </p>
        <div class="mt-7 flex flex-wrap gap-3">
          <Button href="#/peta-wilayah" variant="primary">
            Lihat di Peta Wilayah
            <ArrowRight class="h-4 w-4" strokeWidth={2} />
          </Button>
          <Button href="#/galeri" variant="secondary">Galeri Foto</Button>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- ============================================================ LOKASI / KONTAK ============================================================ -->
<section class="container-page py-16 md:py-20" aria-labelledby="lokasi-title">
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <div class="lg:col-span-2">
      <p class="eyebrow">Lokasi</p>
      <h2
        id="lokasi-title"
        class="mt-3 font-serif text-3xl font-semibold text-arang-900"
      >
        Kantor Desa Gerbosari
      </h2>
      <p class="mt-4 text-arang-700 leading-relaxed max-w-xl">
        Pusat pemerintahan desa berlokasi di Pedukuhan Karang. Jam pelayanan
        mengikuti hari kerja kantor desa.
      </p>

      <dl class="mt-6 space-y-3 text-sm text-arang-700">
        {#if alamat?.kantor}
          <div class="flex items-start gap-3">
            <MapPin class="h-4 w-4 mt-0.5 text-menoreh-600 shrink-0" strokeWidth={1.75} />
            <div>
              <dt class="sr-only">Alamat</dt>
              <dd>
                {alamat.kantor}{#if alamat?.kecamatan}, Kec. {alamat.kecamatan}{/if}{#if alamat?.kabupaten}, {alamat.kabupaten}{/if}{#if alamat?.provinsi}, {alamat.provinsi}{/if}{#if alamat?.kode_pos}&nbsp;{alamat.kode_pos}{/if}
              </dd>
            </div>
          </div>
        {/if}
        {#if alamat?.telepon}
          <div class="flex items-start gap-3">
            <Phone class="h-4 w-4 mt-0.5 text-menoreh-600 shrink-0" strokeWidth={1.75} />
            <div>
              <dt class="sr-only">Telepon</dt>
              <dd class="tnum">{alamat.telepon}</dd>
            </div>
          </div>
        {/if}
      </dl>

      <div class="mt-7">
        <Button href="#/peta-wilayah" variant="primary">
          Buka Peta Interaktif
          <ArrowRight class="h-4 w-4" strokeWidth={2} />
        </Button>
      </div>
    </div>

    <aside class="lg:col-span-1" aria-label="Catatan & sumber">
      {#if highlights.length > 0}
        <Card>
          <h3 class="font-serif text-base font-semibold text-arang-900">Catatan Singkat</h3>
          <ul class="mt-3 space-y-3 text-sm text-arang-700 leading-relaxed">
            {#each highlights.slice(0, 3) as h}
              <li>
                <span class="font-medium text-arang-900">{h.judul}.</span>
                {h.deskripsi}
              </li>
            {/each}
          </ul>
        </Card>
      {/if}
    </aside>
  </div>
</section>
