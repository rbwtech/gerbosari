<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { writable } from 'svelte/store';
  import type { Map as LeafletMap, CircleMarker, LatLngTuple } from 'leaflet';
  import PageHeader from '../lib/components/layout/PageHeader.svelte';
  import SectionShell from '../lib/components/ui/SectionShell.svelte';
  import Badge from '../lib/components/ui/Badge.svelte';
  import Card from '../lib/components/ui/Card.svelte';
  import { getPetaWilayah } from '../lib/content';
  import { MapPin, Mountain, Compass, Coffee, Wheat, Palette, Fish } from '../lib/components/icons';

  // ---------------------------------------------------------------------------
  // Data + boundary metadata
  // ---------------------------------------------------------------------------
  const data = getPetaWilayah() as any;
  const center: LatLngTuple = [data.koordinat.lat, data.koordinat.lon];
  const officeLabel: string = data.koordinat.lokasi ?? 'Kantor Desa Gerbosari';
  const boundaryPolygon: LatLngTuple[] = (data?.wilayah?.polygon ?? []) as LatLngTuple[];
  const batasPoints: Array<{
    arah: string;
    lat: number;
    lon: number;
    berbatasan_dengan: string;
  }> = data?.wilayah?.batas_koordinat ?? [];

  /**
   * Each pedukuhan carries real lat/lon from OpenStreetMap (Nominatim
   * search per name, verified inside Gerbosari's relation 5615721 bbox).
   * Two villages (Kemiriombo, Sendat) are estimated from neighbouring
   * zona-members and marked with `lokasi_perkiraan: true`.
   */
  const pedukuhanPoints = data.pedukuhan
    .filter((p: any) => typeof p.lat === 'number' && typeof p.lon === 'number')
    .map((p: any) => ({
      ...p,
      position: [p.lat, p.lon] as LatLngTuple
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

  $: visiblePedukuhan = pedukuhanPoints.filter((p: any) => {
    const z = p.zona as unknown as ZonaKey | null;
    return z !== null && activeZones.has(z);
  });

  $: groupedByZona = ([1, 2, 3, 4] as ZonaKey[]).map((z) => ({
    nomor: z,
    tema: data.zona.find((d: any) => d.nomor === z)?.tema ?? zonaMeta[z].theme,
    items: pedukuhanPoints.filter((p: any) => (p.zona as unknown as number) === z)
  }));

  // ---------------------------------------------------------------------------
  // Leaflet lifecycle
  // ---------------------------------------------------------------------------
  let mapEl: HTMLDivElement;
  let mapInstance: LeafletMap | undefined;
  let markerByName = new Map<string, CircleMarker>();
  let leafletLib: typeof import('leaflet') | undefined;

  async function initMap() {
    if (typeof window === 'undefined') return;
    const L = await import('leaflet');
    leafletLib = L;

    mapInstance = L.map(mapEl, {
      scrollWheelZoom: true,
      zoomControl: true,
      attributionControl: true
    }).setView(center, 13);

    L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a>',
      maxZoom: 18
    }).addTo(mapInstance);

    L.control.scale({ imperial: false, position: 'bottomleft' }).addTo(mapInstance);

    // Village boundary polygon (scaled visual reconstruction, ~966 ha).
    // Not survey-grade - replace with BPN/BPS GeoJSON when available.
    if (boundaryPolygon.length >= 3) {
      L.polygon(boundaryPolygon, {
        color: '#2f5233',
        weight: 2,
        dashArray: '6 4',
        fillColor: '#2f5233',
        fillOpacity: 0.08
      })
        .bindTooltip('Wilayah Desa Gerbosari (perkiraan visual, ~966,3 ha)', {
          sticky: true,
          direction: 'top'
        })
        .addTo(mapInstance);
    }

    // Cardinal-direction boundary markers (small ring badges via divIcon).
    const batasIconStyle =
      'width:22px;height:22px;border-radius:50%;background:#fff;border:2px solid #244226;' +
      'display:flex;align-items:center;justify-content:center;font-family:Inter,system-ui,sans-serif;' +
      'font-size:10px;font-weight:700;color:#244226;letter-spacing:.04em';
    batasPoints.forEach((b) => {
      const icon = L.divIcon({
        className: 'gerbosari-batas-icon',
        html: `<div style="${batasIconStyle}">${b.arah.charAt(0).toUpperCase()}</div>`,
        iconSize: [22, 22],
        iconAnchor: [11, 11]
      });
      L.marker([b.lat, b.lon], { icon })
        .bindPopup(
          `<div style="font-family:Inter,system-ui,sans-serif;min-width:200px"><div style="font-size:10px;letter-spacing:.16em;text-transform:uppercase;color:#9a4a2c;font-weight:600">Batas ${escapeHtml(b.arah)}</div><div style="margin-top:4px;color:#1a1816;font-weight:500;line-height:1.4">${escapeHtml(b.berbatasan_dengan)}</div><div style="margin-top:6px;font-size:11px;color:#7a6a5c;font-family:'JetBrains Mono',ui-monospace,monospace">${b.lat.toFixed(6)}, ${b.lon.toFixed(6)}</div></div>`
        )
        .addTo(mapInstance!);
    });

    // Kantor desa (office) - distinct ring marker
    L.circleMarker(center, {
      radius: 10,
      color: '#b85c38',
      weight: 3,
      fillColor: '#fdf5f0',
      fillOpacity: 1
    })
      .bindPopup(
        `<div style="font-family:Inter,system-ui,sans-serif;min-width:220px"><div style="font-size:10px;letter-spacing:.16em;text-transform:uppercase;color:#9a4a2c;font-weight:600">Kantor Kelurahan</div><div style="margin-top:4px;font-weight:600;color:#1a1816">Desa Gerbosari</div><div style="margin-top:4px;color:#52493f;font-size:12px;line-height:1.5">${escapeHtml(officeLabel)}</div><div style="margin-top:6px;font-size:11px;color:#7a6a5c;font-family:'JetBrains Mono',ui-monospace,monospace">${center[0].toFixed(6)}, ${center[1].toFixed(6)}</div></div>`
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

    visiblePedukuhan.forEach((p: any) => {
      const z = p.zona as unknown as ZonaKey | null;
      const color = z ? zonaMeta[z].color : '#52493f';
      const estimasi = p.lokasi_perkiraan === true;
      const m = L.circleMarker(p.position as LatLngTuple, {
        radius: 7,
        color,
        weight: 2,
        fillColor: color,
        fillOpacity: estimasi ? 0.3 : 0.55,
        dashArray: estimasi ? '3 2' : undefined
      }).bindPopup(
        `<div style="font-family:Inter,system-ui,sans-serif;min-width:200px"><div style="font-weight:600;color:#1a1816">${escapeHtml(p.nama)}</div><div style="margin-top:2px;font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:#9a4a2c">${z ? `Zona ${romanZona[z]}` : 'Transisi'}${estimasi ? ' &middot; lokasi perkiraan' : ''}</div>${p.deskripsi ? `<div style="margin-top:6px;color:#52493f;font-size:12px;line-height:1.5">${escapeHtml(p.deskripsi)}</div>` : ''}</div>`
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
    batasPoints.forEach((b) => bounds.push([b.lat, b.lon]));
    boundaryPolygon.forEach((p) => bounds.push(p));
    mapInstance.fitBounds(bounds, { padding: [40, 40], maxZoom: 14 });
  }

  function focusPedukuhan(nama: string, scroll = false) {
    if (!mapInstance) return;
    const m = markerByName.get(nama);
    if (!m) return;
    selected.set(nama);
    mapInstance.setView(m.getLatLng(), 14, { animate: true });
    m.openPopup();
    // When the user clicks a pedukuhan in the list below the map, the map is
    // typically scrolled off-screen on mobile - bring it back into view so the
    // marker open animation is actually seen.
    if (scroll && mapEl && typeof window !== 'undefined') {
      mapEl.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
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

  // Quick facts - present both luas figures, larger primary, smaller as tooltip.
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
    { label: 'Ketinggian', value: `${data.ketinggian_min}-${data.ketinggian_max} m dpl` }
  ];

  // Suroloyo profile - pulled from JSON's wisata_unggulan field
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
  description="Lereng perbukitan Menoreh, ketinggian 400-900 m dpl, 19 pedukuhan terbagi dalam 4 kawasan pembangunan."
/>

<!-- Quick facts on hijau mist - signals geography section. 2-col grid on
     phones (5 items wrap to 3 rows), promotes to 5-col on desktop. -->
<SectionShell variant="mist" padding="sm">
  <ul class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-2 sm:gap-3">
    {#each quickFacts as fact}
      <li
        class="rounded-lg border border-krem-200 bg-white px-3 py-2.5 sm:px-4 sm:py-3 min-w-0"
        title={fact.hint ?? ''}
      >
        <div class="text-[10px] sm:text-[11px] font-semibold uppercase tracking-[0.14em] text-terakota-600 truncate">{fact.label}</div>
        <div class="mt-1 text-base sm:text-lg font-semibold text-arang-900 tnum break-words">{fact.value}</div>
      </li>
    {/each}
  </ul>
</SectionShell>

<!-- Map: white wrapper with overflow-hidden so Leaflet tiles cannot spill
     past the SectionShell into the sidebar/viewport edge on narrow screens. -->
<div class="container-page py-8 md:py-10 overflow-hidden">
  <div class="rounded-xl border border-krem-200 bg-white overflow-hidden w-full max-w-full">
    <div
      bind:this={mapEl}
      class="w-full max-w-full h-[55vh] sm:h-[480px] md:h-[520px] lg:h-[560px] rounded-lg overflow-hidden border border-krem-200"
      aria-label="Peta interaktif Desa Gerbosari"
    ></div>
  </div>
</div>

<!-- Legend + filter chips strip. -->
<SectionShell variant="default" padding="sm">
  <div>
    <span class="text-xs font-semibold uppercase tracking-[0.14em] text-arang-700/70 block mb-2">Filter zona</span>
    <!-- 2-col grid on mobile (no scroll), wraps inline on desktop. -->
    <div class="grid grid-cols-2 sm:flex sm:flex-wrap gap-2">
      {#each [1, 2, 3, 4] as z (z)}
        {@const key = z as ZonaKey}
        {@const active = activeZones.has(key)}
        {@const count = pedukuhanPoints.filter((p: any) => (p.zona as unknown as number) === z).length}
        <button
          type="button"
          on:click={() => toggleZone(key)}
          class="inline-flex items-center gap-2 min-h-11 h-11 pl-2 pr-3 rounded-full text-sm border transition-colors {active
            ? 'bg-arang-900 text-krem-50 border-arang-900'
            : 'bg-white text-arang-700 border-krem-200 hover:border-arang-300'}"
          aria-pressed={active}
        >
          <span
            class="inline-block w-3 h-3 rounded-full shrink-0"
            style="background:{zonaMeta[key].color}"
            aria-hidden="true"
          ></span>
          <span class="truncate">Zona {romanZona[key]}</span>
          <span class="text-[11px] opacity-70 tnum ml-auto">· {count}</span>
        </button>
      {/each}
    </div>
  </div>
  <!-- Batas wilayah ringkas - compact summary alongside the legend. -->
  <Card padding="md" class="mt-8 bg-krem-50">
    <div class="flex items-center gap-2 mb-4">
      <Compass class="w-4 h-4 text-terakota-600" strokeWidth={1.75} />
      <span class="text-xs font-semibold uppercase tracking-[0.14em] text-terakota-600">Batas Wilayah</span>
    </div>
    <dl class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 text-sm">
      {#each [['Utara', data?.batas?.utara], ['Timur', data?.batas?.timur], ['Selatan', data?.batas?.selatan], ['Barat', data?.batas?.barat]] as [arah, isi]}
        <div>
          <dt class="text-[11px] font-semibold uppercase tracking-[0.14em] text-arang-700/60">{arah}</dt>
          <dd class="mt-1 text-arang-900">{isi}</dd>
        </div>
      {/each}
    </dl>
  </Card>
</SectionShell>


<!-- Daftar Pedukuhan on batik motif - each zona column carries an accent border-top. -->
<SectionShell variant="batik" padding="lg">
  <div class="mb-6">
    <p class="eyebrow">Pembagian Kawasan</p>
    <h2 class="mt-2 font-serif text-2xl md:text-3xl font-semibold text-arang-900">Daftar Pedukuhan</h2>
    <p class="mt-2 text-arang-700 max-w-2xl">
      Empat zona pembangunan yang mengelompokkan sembilan belas pedukuhan menurut potensi pertanian, kebudayaan, dan kerajinan.
    </p>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-5">
    {#each groupedByZona as group (group.nomor)}
      {@const meta = zonaMeta[group.nomor]}
      {@const Icon = meta.icon}
      {@const dim = !activeZones.has(group.nomor)}
      <div
        class="rounded-xl border border-krem-200 bg-white overflow-hidden transition-opacity {dim ? 'opacity-50' : ''}"
        style="border-top:3px solid {meta.color}"
      >
        <header class="flex items-start gap-3 p-4 sm:p-5 border-b border-krem-200">
          <span
            class="font-serif text-4xl leading-none font-semibold flex-none"
            style="color:{meta.color}"
            aria-hidden="true">{romanZona[group.nomor]}</span
          >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 text-[11px] font-semibold uppercase tracking-[0.14em] text-arang-700/70">
              <Icon class="w-3.5 h-3.5 flex-none" strokeWidth={1.75} />
              <span class="break-words">{zonaMeta[group.nomor].theme}</span>
            </div>
            <p class="mt-1 text-sm text-arang-700 leading-snug line-clamp-3 break-words">{group.tema}</p>
          </div>
        </header>
        <ul class="divide-y divide-krem-200">
          {#each group.items as p (p.nama)}
            <li>
              <button
                type="button"
                on:click={() => focusPedukuhan(p.nama, true)}
                on:mouseenter={() => focusPedukuhan(p.nama)}
                class="w-full text-left px-4 sm:px-5 py-3 min-h-11 hover:bg-krem-50 transition-colors {$selected === p.nama ? 'bg-krem-100' : ''}"
              >
                <div class="flex items-center justify-between gap-3">
                  <span class="font-medium text-arang-900 break-words min-w-0">{p.nama}</span>
                  <MapPin class="w-3.5 h-3.5 text-arang-700/40 flex-none" strokeWidth={1.75} />
                </div>
                {#if p.deskripsi}
                  <p class="mt-1 text-xs text-arang-700/80 leading-relaxed line-clamp-2 break-words">{p.deskripsi}</p>
                {/if}
              </button>
            </li>
          {/each}
          {#if group.items.length === 0}
            <li class="px-4 sm:px-5 py-4 text-xs text-arang-700/60 italic">Belum ada pedukuhan terdata.</li>
          {/if}
        </ul>
      </div>
    {/each}
  </div>
</SectionShell>

<!-- Profil Suroloyo: warm tanah-paper biome befits the heritage / kahyangan tone. -->
{#if suroloyo}
  <SectionShell variant="tanah" padding="lg">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 lg:gap-12 items-start">
      <figure class="rounded-xl overflow-hidden border border-krem-200 bg-krem-100 aspect-[4/3]">
        <img
          src="/images/gallery/gerbosari-puncak-suroloyo.png"
          alt="Gardu pandang Puncak Suroloyo"
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
          {suroloyo?.nama ?? ''}
        </h2>
        <div class="mt-3 flex flex-wrap items-center gap-2 text-sm text-arang-700">
          <Badge variant="wisata">{suroloyo.ketinggian_m_dpl} m dpl</Badge>
          <span>·</span>
          <span>Pedukuhan {suroloyo.lokasi_pedukuhan}</span>
        </div>
        <p class="mt-4 text-arang-700 leading-relaxed">{suroloyo?.deskripsi ?? ''}</p>

        <div class="mt-6">
          <h3 class="text-sm font-semibold uppercase tracking-[0.14em] text-arang-700/70">
            Yang bisa dilihat dari puncak
          </h3>
          <ul class="mt-3 space-y-2">
            {#each arahPandang as item}
              <li class="flex gap-3 text-sm">
                <span
                  class="flex-none w-14 sm:w-16 font-medium text-terakota-700 uppercase text-[11px] tracking-[0.14em] pt-0.5"
                  >{item.arah}</span
                >
                <span class="text-arang-800 leading-relaxed break-words min-w-0">{item.isi}</span>
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
    </div>
  </SectionShell>
{/if}

<style>
  :global(.leaflet-container) {
    width: 100% !important;
    max-width: 100%;
    font-family: 'Inter', system-ui, sans-serif;
    background: #f5ebe0;
    z-index: 0;
  }
  :global(.leaflet-pane),
  :global(.leaflet-top),
  :global(.leaflet-bottom) {
    z-index: 1 !important;
  }
  :global(.leaflet-popup-pane) {
    z-index: 2 !important;
  }
  :global(.leaflet-popup-content-wrapper) {
    border-radius: 8px;
    box-shadow: 0 4px 16px rgba(26, 24, 22, 0.12);
  }
  :global(.leaflet-popup-content) {
    margin: 12px 14px;
    max-width: 240px;
  }
  :global(.leaflet-control-attribution) {
    background: rgba(255, 255, 255, 0.85);
    font-size: 10px;
  }
</style>
