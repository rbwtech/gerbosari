<script lang="ts">
  import { link } from '../../router';
  import { MapPin, Phone, Award } from '../icons';

  const tahun = new Date().getFullYear();

  const navigasi = [
    { href: '/sejarah', label: 'Sejarah' },
    { href: '/visi-misi', label: 'Visi & Misi' },
    { href: '/struktur-organisasi', label: 'Struktur Organisasi' },
    { href: '/peta-wilayah', label: 'Peta Wilayah' },
    { href: '/galeri', label: 'Galeri' },
    { href: '/data-penduduk', label: 'Data Penduduk' },
    { href: '/lowongan', label: 'Lowongan Kerja' },
    { href: '/berita', label: 'Berita & Agenda' }
  ];
</script>

<footer class="mt-24 bg-menoreh-900 text-krem-100">
  <div class="container-page py-14 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10">
    <!-- Identitas desa -->
    <div class="min-w-0">
      <div class="flex items-center gap-2.5">
        <svg viewBox="0 0 32 32" class="w-8 h-8" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="22" cy="9" r="2.25" class="text-terakota-400" fill="currentColor" stroke="none" />
          <path d="M3 25 L11 13 L17 20 L23 11 L29 25 Z" class="text-krem-50" />
          <path d="M3 25 L29 25" class="text-krem-50" />
        </svg>
        <span class="font-serif text-base font-semibold text-krem-50">Desa Gerbosari</span>
      </div>
      <p class="mt-4 font-serif text-base italic text-krem-100/85 leading-relaxed text-pretty">
        Tanah Menoreh, ladang budaya, rumah seribu cerita.
      </p>
      <p class="mt-3 text-sm text-krem-100/70 leading-relaxed text-pretty">
        Profil resmi Pemerintah Desa Gerbosari, Kecamatan Samigaluh, Kabupaten
        Kulon Progo, Daerah Istimewa Yogyakarta.
      </p>
    </div>

    <!-- Kontak -->
    <div class="min-w-0">
      <h4 class="text-xs font-semibold uppercase tracking-widest text-krem-50">Kontak</h4>
      <ul class="mt-4 space-y-3 text-sm">
        <li class="flex gap-3 text-krem-100/85 min-w-0">
          <MapPin class="w-4 h-4 mt-0.5 shrink-0 text-terakota-400" strokeWidth={1.75} aria-hidden="true" />
          <!-- hyphens-auto allows graceful break of "Pangaji-Plono" on narrow viewports -->
          <span class="leading-relaxed break-words [hyphens:auto]">
            Jl. Pangaji-Plono KM 4.2, Karang,<br />
            Gerbosari, Samigaluh,<br />
            Kulon Progo, DIY 55673
          </span>
        </li>
        <li class="flex gap-3 text-krem-100/85">
          <Phone class="w-4 h-4 mt-0.5 shrink-0 text-terakota-400" strokeWidth={1.75} aria-hidden="true" />
          <a href="tel:+622742820979" class="hover:text-krem-50 transition-colors duration-200 ease-out tnum">
            (0274) 2820979
          </a>
        </li>
      </ul>
    </div>

    <!-- Navigasi -->
    <div class="min-w-0">
      <h4 class="text-xs font-semibold uppercase tracking-widest text-krem-50">Navigasi</h4>
      <ul class="mt-4 grid grid-cols-1 gap-1 text-sm">
        {#each navigasi as item}
          <li>
            <a
              href={item.href}
              use:link
              class="inline-flex items-center min-h-9 py-1 text-krem-100/80 hover:text-krem-50 transition-colors duration-200 ease-out break-words"
            >
              {item.label}
            </a>
          </li>
        {/each}
      </ul>
    </div>

    <!-- Akreditasi -->
    <div class="min-w-0">
      <h4 class="text-xs font-semibold uppercase tracking-widest text-krem-50">Penghargaan</h4>
      <div class="mt-4 flex gap-3 items-start">
        <Award class="w-5 h-5 mt-0.5 shrink-0 text-terakota-400" strokeWidth={1.75} aria-hidden="true" />
        <div class="text-sm text-krem-100/85 leading-relaxed min-w-0">
          <div class="font-medium text-krem-50">Juara II</div>
          <div class="break-words">Lomba Desa Tingkat Daerah Istimewa Yogyakarta</div>
          <div class="mt-1 text-xs uppercase tracking-widest text-krem-100/60">Tahun 2018</div>
        </div>
      </div>
    </div>
  </div>

  <div class="border-t border-krem-100/10">
    <div class="container-page py-5 text-xs text-krem-100/60 flex flex-col gap-3 sm:flex-row sm:justify-between sm:items-center">
      <span class="break-words">&copy; {tahun} Pemerintah Desa Gerbosari. Hak cipta dilindungi.</span>
      <span class="flex flex-wrap items-center gap-x-3 gap-y-1">
        <a
          href="#/admin/login"
          class="inline-flex items-center min-h-9 text-xs text-krem-100/60 hover:text-terakota-300 transition-colors duration-200 ease-out"
        >
          Admin
        </a>
      </span>
    </div>
  </div>
</footer>
