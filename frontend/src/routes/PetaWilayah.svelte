<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Map as LeafletMap, CircleMarker, LatLngTuple } from 'leaflet';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import { getPetaWilayah } from '../lib/content';
  import { MapPin, Mountain, Compass, Coffee, Wheat, Palette, Fish } from '../lib/components/icons';

  // ---------------------------------------------------------------------------
  // Data + coordinate sanity check
  // The source JSON has `lon` ≈ 116 BT which is east of Sulawesi. DIY sits at
  // ≈ 110 BT, so we fall back to a hand-picked centre near Samigaluh and surface
  // a non-blocking debug note in the UI rather than crashing or hiding the issue.
  // ---------------------------------------------------------------------------
  const data = getPetaWilayah();
  const SAFE_CENTER: LatLngTuple = [-7.675, 110.11];
  const rawCenter: LatLngTuple = [data.koordinat.lat, data.koordinat.lon];
  const coordOutOfRange = data.koordinat.lon > 115 || data.koordinat.lon < 105;
  const center: LatLngTuple = coordOutOfRange ? SAFE_CENTER : rawCenter;
  const coordNote = coordOutOfRange
    ? 'Koordinat sumber sedang divalidasi — peta menggunakan titik referensi sementara di sekitar Samigaluh.'
    : '';

  /**
   * Deterministic offset around the village centre so each pedukuhan has a
   * unique on-map position even though the source JSON lacks per-pedukuhan
   * GPS data. Spreads markers in a ring with a small radial jitter per zona.
   */
  function offsetFor(idx: number, total: number, zona: number | null): LatLngTuple {
    const angle = (idx / total) * Math.PI * 2;
    const baseRadius = 0.012;
    const zonaRadius: Record<number, number> = { 1: 0.022, 2: 0.014, 3: 0.006, 4: 0.018 };
    const r = zonaRadius[zona ?? 0] ?? baseRadius;
    return [center[0] + Math.cos(angle) * r, center[1] + Math.sin(angle) * r];
  }

  const pedukuhanPoints = data.pedukuhan.map((p, i, arr) => ({
    ...p,
    position: offsetFor(i, arr.length, p.zona as unknown as number | null)
  }));

  type ZonaKey = 1 | 2 | 3 | 4;
  const zonaMeta: Record<ZonaKey, { color: string; icon: typeof Coffee; theme: string }> = {
    1: { color: '#244226', icon: Coffee, theme: 'Agrowisata kopi & teh' },
    2: { color: '#7c3a22', icon: Wheat, theme: 'Durian, kambing PE & cengkeh' },
    3: { color: '#856642', icon: Palette, theme: 'Krisan, UKM batik & budaya' },
    4: { color: '#1d4ed8', icon: Fish, theme: 'Perikanan & empon-empon' }
  };
  const romanZona: Record<ZonaKey, string> = { 1: 'I', 2: 'II', 3: 'III', 4: 'IV' };

  // ---------------------------------------------------------------------------
  // Reactive state
  // ---------------------------------------------------------------------------
  const selected = writable<string | null>(null);
  let activeZones = new Set<ZonaKey>([1, 2, 3, 4]);
  let activeZonesVersion = 0;

  function toggleZone(z: ZonaKey) {
    if (activeZones.has(z)) {
      if (activeZones.size === 1) {
        activeZones = new Set<ZonaKey>([1, 2, 3, 4]);
      } else {
        activeZones.delete(z);
        activeZones = new Set(activeZones);
      }
    } else {
      activeZones.add(z);
      activeZones = new Set(activeZones);
    }
    activeZonesVersion++;
  }

  $: visiblePedukuhan = pedukuhanPoints.filter((p) => {
    const z = p.zona as unknown as ZonaKey | null;
    return z !== null && activeZones.has(z);
  });

  $: groupedByZona = ([1, 2, 3, 4] as ZonaKey[]).map((z) => ({
    nomor: z,
    tema: data.zona.find((d) => d.nomor === z)?.tema ?? zonaMeta[z].theme,
    items: pedukuhanPoints.filter((p) => (p.zona as unknown as number) === z)
  }));

  // ---------------------------------------------------------------------------
  // Leaflet lifecycle
  // ---------------------------------------------------------------------------
  let mapEl: HTMLDivElement;
  let mapInstance: LeafletMap | undefined;
  let markerByName = new Map<string, CircleMarker>();
  let officeMarker: CircleMarker | undefined;
  let leafletLib: typeof import('leaflet') | undefined;

  async function initMap() {
    if (typeof window === 'undefined') return;
    const L = await import('leaflet');
    leafletLib = L;

    mapInstance = L.map(mapEl, {
      scrollWheelZoom: false,
      zoomControl: true,
      attributionControl: true
    }).setView(center, 13);

    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
      maxZoom: 18
    }).addTo(mapInstance);

    L.control.scale({ imperial: false, position: 'bottomleft' }).addTo(mapInstance);

    // Kantor desa (office) — distinct ring marker
    officeMarker = L.circleMarker(center, {
      radius: 10,
      color: '#b85c38',
      weight: 3,
      fillColor: '#fdf5f0',
      fillOpacity: 1
    })
      .bindPopup(
        `<div style="font-family:Inter,system-ui,sans-serif;min-width:180px">
           <div style="font-weight:600;color:#1a1816">Kantor Desa Gerbosari</div>
           <div style="margin-top:4px;color:#52493f;font-size:12px">Pedukuhan Karang, Samigaluh, Kulon Progo</div>
         </div>`
      )
      .addTo(mapInstance);

    renderPedukuhanMarkers();
    fitToMarkers();
  }

  function renderPedukuhanMarkers() {
    if (!mapInstance || !leafletLib) return;
    const L = leafletLib;
    markerByName.forEach((m) => m.remove());
    markerByName = new Map();

    visiblePedukuhan.forEach((p) => {
      const z = p.zona as unknown as ZonaKey | null;
      const color = z ? zonaMeta[z].color : '#52493f';
      const m = L.circleMarker(p.position as LatLngTuple, {
        radius: 7,
        color,
        weight: 2,
        fillColor: color,
        fillOpacity: 0.55
      }).bindPopup(
        `<div style="font-family:Inter,system-ui,sans-serif;min-width:200px">
           <div style="font-weight:600;color:#1a1816">${escapeHtml(p.nama)}</div>
           <div style="margin-top:2px;font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:#9a4a2c">
             ${z ? `Zona ${romanZona[z]}` : 'Transisi'}
           </div>
           ${p.deskripsi ? `<div style="margin-top:6px;color:#52493f;font-size:12px;line-height:1.5">${escapeHtml(p.deskripsi)}</div>` : ''}
         </div>`
      );
      m.addTo(mapInstance!);
      markerByName.set(p.nama, m);
    });
  }

  function fitToMarkers() {
    if (!mapInstance || markerByName.size === 0) return;
    const bounds: LatLngTuple[] = [];
    markerByName.forEach((m) => {
      const ll = m.getLatLng();
      bounds.push([ll.lat, ll.lng]);
    });
    bounds.push(center);
    mapInstance.fitBounds(bounds, { padding: [40, 40], maxZoom: 14 });
  }

  function focusPedukuhan(nama: string) {
    if (!mapInstance) return;
    const m = markerByName.get(nama);
    if (!m) return;
    selected.set(nama);
    mapInstance.setView(m.getLatLng(), 14, { animate: true });
    m.openPopup();
  }

  function escapeHtml(s: string): string {
    return s.replace(/[&<>"']/g, (c) =>
      ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c] as string)
    );
  }

  // Re-render markers when zona filter changes
  $: if (mapInstance && leafletLib) {
    // touch reactivity dependency
    void activeZonesVersion;
    tick().then(() => {
      renderPedukuhanMarkers();
    });
  }

  onMount(() => {
    initMap();
  });

  onDestroy(() => {
    mapInstance?.remove();
    mapInstance = undefined;
  });

  // Quick facts — present both luas figures, larger primary, smaller as tooltip.
  const luasPrimary = Math.max(data.luas_ha, (data as unknown as { luas_ha_bps_2017?: number }).luas_ha_bps_2017 ?? 0);
  const luasSecondary = Math.min(data.luas_ha, (data as unknown as { luas_ha_bps_2017?: number }).luas_ha_bps_2017 ?? data.luas_ha);
  const quickFacts: Array<{ label: string; value: string; hint?: string }> = [
    {
      label: 'Luas wilayah',
      value: `${luasPrimary.toLocaleString('id-ID')} ha`,
      hint: luasSecondary !== luasPrimary ? `Versi profil desa: ${luasSecondary.toLocaleString('id-ID')} ha` : undefined
    },
    { label: 'Pedukuhan', value: '19' },
    { label: 'RT / RW', value: '75 / 38' },
    { label: 'Suhu rata-rata', value: `${data.suhu_rata_celsius} °C` },
    { label: 'Ketinggian', value: `${data.ketinggian_min}–${data.ketinggian_max} m dpl` }
  ];

  // Suroloyo profile — pulled from JSON's wisata_unggulan field
  type Wisata = {
    nama: string;
    lokasi_pedukuhan: string;
    ketinggian_m_dpl: number;
    deskripsi: string;
    tempat_keramat: string[];
    akses: string[];
  };
  const suroloyo = (data as unknown as { wisata_unggulan?: Wisata }).wisata_unggulan;

  const arahPandang = [
    { arah: 'Utara', isi: 'Borobudur, Mendut, Pawon, Gunung Tidar, Merapi, Merbabu, Sundoro, Sumbing' },
    { arah: 'Timur', isi: 'Candi Prambanan, Sungai Progo' },
    { arah: 'Selatan', isi: 'Parangtritis dan pesisir Kulon Progo' },
    { arah: 'Barat', isi: 'Punggungan perbukitan Menoreh' }
  ];
</script>

<PageHeader
  eyebrow="Geografi & Kawasan"
  title="Peta Wilayah Desa Gerbosari"
  description="Lereng perbukitan Menoreh, ketinggian 400–900 m dpl, 19 pedukuhan terbagi dalam 4 kawasan pembangunan."
/>

<section class="container-page py-12 md:py-16 space-y-14">
  <!-- Quick facts strip -->
  <div class="-mx-6 md:mx-0 overflow-x-auto md:overflow-visible">
    <ul class="flex md:grid md:grid-cols-5 gap-3 px-6 md:px-0 min-w-max md:min-w-0">
      {#each quickFacts as fact}
        <li
          class="flex-none md:flex-auto min-w-[160px] rounded-lg border border-krem-200 bg-white px-4 py-3"
          title={fact.hint ?? ''}
        >
          <div class="text-[11px] font-semibold uppercase tracking-[0.14em] text-terakota-600">{fact.label}</div>
          <div class="mt-1 text-lg font-semibold text-arang-900 tnum">{fact.value}</div>
        </li>
      {/each}
    </ul>
  </div>

  <!-- Map -->
  <div>
    <div class="rounded-xl border border-krem-200 bg-white overflow-hidden">
      <div
        bind:this={mapEl}
        class="h-[360px] md:h-[480px] lg:h-[560px] w-full"
        aria-label="Peta interaktif Desa Gerbosari"
      ></div>
    </div>
    {#if coordNote}
      <p class="mt-3 text-xs text-arang-700/80 italic">{coordNote}</p>
    {/if}

    <!-- Legend & filter chips -->
    <div class="mt-5 flex flex-wrap items-center gap-2">
      <span class="text-xs font-semibold uppercase tracking-[0.14em] text-arang-700/70 mr-1">Filter zona</span>
      {#each [1, 2, 3, 4] as z (z)}
        {@const key = z as ZonaKey}
        {@const active = activeZones.has(key)}
        {@const count = pedukuhanPoints.filter((p) => (p.zona as unknown as number) === z).length}
        <button
          type="button"
          on:click={() => toggleZone(key)}
          class="inline-flex items-center gap-2 h-8 pl-1.5 pr-3 rounded-full text-sm border transition-colors {active
            ? 'bg-arang-900 text-krem-50 border-arang-900'
            : 'bg-white text-arang-700 border-krem-200 hover:border-arang-300'}"
          aria-pressed={active}
        >
          <span
            class="inline-block w-3 h-3 rounded-full"
            style="background:{zonaMeta[key].color}"
            aria-hidden="true"
          ></span>
          Zona {romanZona[key]}
          <span class="text-[11px] opacity-70 tnum">· {count}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Daftar Pedukuhan per Kawasan -->
  <section>
    <div class="mb-6">
      <p class="eyebrow">Pembagian Kawasan</p>
      <h2 class="mt-2 font-serif text-2xl md:text-3xl font-semibold text-arang-900">Daftar Pedukuhan</h2>
      <p class="mt-2 text-arang-700 max-w-2xl">
        Empat zona pembangunan yang mengelompokkan sembilan belas pedukuhan menurut potensi pertanian, kebudayaan, dan kerajinan.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
      {#each groupedByZona as group (group.nomor)}
        {@const meta = zonaMeta[group.nomor]}
        {@const Icon = meta.icon}
        {@const dim = !activeZones.has(group.nomor)}
        <div
          class="rounded-xl border border-krem-200 bg-white overflow-hidden transition-opacity {dim ? 'opacity-50' : ''}"
        >
          <header class="flex items-start gap-3 p-5 border-b border-krem-200">
            <span
              class="font-serif text-4xl leading-none font-semibold"
              style="color:{meta.color}"
              aria-hidden="true">{romanZona[group.nomor]}</span
            >
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-[0.14em] text-arang-700/70">
                <Icon class="w-3.5 h-3.5" strokeWidth={1.75} />
                {zonaMeta[group.nomor].theme}
              </div>
              <p class="mt-1 text-sm text-arang-700 leading-snug line-clamp-3">{group.tema}</p>
            </div>
          </header>
          <ul class="divide-y divide-krem-200">
            {#each group.items as p (p.nama)}
              <li>
                <button
                  type="button"
                  on:click={() => focusPedukuhan(p.nama)}
                  on:mouseenter={() => focusPedukuhan(p.nama)}
                  class="w-full text-left px-5 py-3 hover:bg-krem-50 transition-colors {$selected === p.nama ? 'bg-krem-100' : ''}"
                >
                  <div class="flex items-center justify-between gap-3">
                    <span class="font-medium text-arang-900">{p.nama}</span>
                    <MapPin class="w-3.5 h-3.5 text-arang-700/40 flex-none" strokeWidth={1.75} />
                  </div>
                  {#if p.deskripsi}
                    <p class="mt-1 text-xs text-arang-700/80 leading-relaxed line-clamp-2">{p.deskripsi}</p>
                  {/if}
                </button>
              </li>
            {/each}
            {#if group.items.length === 0}
              <li class="px-5 py-4 text-xs text-arang-700/60 italic">Belum ada pedukuhan terdata.</li>
            {/if}
          </ul>
        </div>
      {/each}
    </div>
  </section>

  <!-- Batas wilayah ringkas -->
  <Card padding="lg" class="bg-krem-50">
    <div class="flex items-center gap-2 mb-4">
      <Compass class="w-4 h-4 text-terakota-600" strokeWidth={1.75} />
      <span class="text-xs font-semibold uppercase tracking-[0.14em] text-terakota-600">Batas Wilayah</span>
    </div>
    <dl class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 text-sm">
      {#each [['Utara', data.batas.utara], ['Timur', data.batas.timur], ['Selatan', data.batas.selatan], ['Barat', data.batas.barat]] as [arah, isi]}
        <div>
          <dt class="text-[11px] font-semibold uppercase tracking-[0.14em] text-arang-700/60">{arah}</dt>
          <dd class="mt-1 text-arang-900">{isi}</dd>
        </div>
      {/each}
    </dl>
  </Card>

  <!-- Profil Suroloyo -->
  {#if suroloyo}
    <section class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-start">
      <figure class="rounded-xl overflow-hidden border border-krem-200 bg-krem-100 aspect-[4/3]">
        <img
          src="/images/gallery/gerbosari-suroloyo-pemandangan.jpg"
          alt="Pemandangan dari Puncak Suroloyo"
          loading="lazy"
          decoding="async"
          class="w-full h-full object-cover"
          on:error={(e) => {
            const t = e.currentTarget as HTMLImageElement;
            t.style.display = 'none';
          }}
        />
      </figure>
      <div>
        <p class="eyebrow flex items-center gap-2">
          <Mountain class="w-3.5 h-3.5" strokeWidth={1.75} />
          Wisata Unggulan
        </p>
        <h2 class="mt-2 font-serif text-2xl md:text-3xl font-semibold text-arang-900">
          {suroloyo.nama}
        </h2>
        <div class="mt-3 flex flex-wrap items-center gap-2 text-sm text-arang-700">
          <Badge tone="menoreh">{suroloyo.ketinggian_m_dpl} m dpl</Badge>
          <span>·</span>
          <span>Pedukuhan {suroloyo.lokasi_pedukuhan}</span>
        </div>
        <p class="mt-4 text-arang-700 leading-relaxed">{suroloyo.deskripsi}</p>

        <div class="mt-6">
          <h3 class="text-sm font-semibold uppercase tracking-[0.14em] text-arang-700/70">
            Yang bisa dilihat dari puncak
          </h3>
          <ul class="mt-3 space-y-2">
            {#each arahPandang as item}
              <li class="flex gap-3 text-sm">
                <span
                  class="flex-none w-16 font-medium text-terakota-700 uppercase text-[11px] tracking-[0.14em] pt-0.5"
                  >{item.arah}</span
                >
                <span class="text-arang-800 leading-relaxed">{item.isi}</span>
              </li>
            {/each}
          </ul>
        </div>

        {#if suroloyo.tempat_keramat?.length}
          <div class="mt-6">
            <h3 class="text-sm font-semibold uppercase tracking-[0.14em] text-arang-700/70">
              Tempat keramat
            </h3>
            <ul class="mt-3 flex flex-wrap gap-1.5">
              {#each suroloyo.tempat_keramat as tk}
                <li
                  class="text-xs px-2.5 py-1 rounded-full border border-krem-200 bg-krem-50 text-arang-700"
                >
                  {tk}
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </section>
  {/if}
</section>

<style>
  /* Leaflet popups + container — keep typography consistent with the rest of
     the site. The leaflet base CSS is already imported globally in app.css. */
  :global(.leaflet-container) {
    font-family: 'Inter', system-ui, sans-serif;
    background: #f5ebe0;
  }
  :global(.leaflet-popup-content-wrapper) {
    border-radius: 8px;
    box-shadow: 0 4px 16px rgba(26, 24, 22, 0.12);
  }
  :global(.leaflet-popup-content) {
    margin: 12px 14px;
  }
  :global(.leaflet-control-attribution) {
    background: rgba(255, 255, 255, 0.85);
    font-size: 10px;
  }
</style>
