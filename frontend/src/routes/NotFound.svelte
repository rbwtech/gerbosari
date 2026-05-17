<script lang="ts">
  import { link } from 'svelte-spa-router';
  import { ChevronLeft } from '../lib/components/icons';
</script>

<section class="container-page py-24 text-center">
  <p class="text-xs font-semibold uppercase tracking-[0.18em] text-terakota-600">Galat 404</p>
  <h1 class="mt-3 font-serif text-4xl md:text-5xl font-semibold text-arang-900">Halaman tidak ditemukan</h1>
  <p class="mt-4 text-arang-700 max-w-md mx-auto">
    Tautan yang Anda buka mungkin sudah dipindahkan atau tidak pernah ada.
  </p>
  <a
    href="/"
    use:link
    class="mt-8 inline-flex items-center gap-2 h-11 px-5 rounded-md bg-menoreh-500 text-krem-50 font-medium text-sm hover:bg-menoreh-600 transition-colors"
  >
    <ChevronLeft class="w-4 h-4" strokeWidth={2} />
    Kembali ke Beranda
  </a>
</section>
