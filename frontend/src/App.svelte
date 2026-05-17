<script lang="ts">
  import Router from 'svelte-spa-router';
  import Navbar from './lib/components/layout/Navbar.svelte';
  import Footer from './lib/components/layout/Footer.svelte';

  import Beranda from './routes/Beranda.svelte';
  import Sejarah from './routes/Sejarah.svelte';
  import VisiMisi from './routes/VisiMisi.svelte';
  import StrukturOrganisasi from './routes/StrukturOrganisasi.svelte';
  import PetaWilayah from './routes/PetaWilayah.svelte';
  import Galeri from './routes/Galeri.svelte';
  import DataPenduduk from './routes/DataPenduduk.svelte';
  import Lowongan from './routes/Lowongan.svelte';
  import Berita from './routes/Berita.svelte';
  import BeritaDetail from './routes/BeritaDetail.svelte';
  import NotFound from './routes/NotFound.svelte';

  const routes = {
    '/': Beranda,
    '/sejarah': Sejarah,
    '/visi-misi': VisiMisi,
    '/struktur-organisasi': StrukturOrganisasi,
    '/peta-wilayah': PetaWilayah,
    '/galeri': Galeri,
    '/data-penduduk': DataPenduduk,
    '/lowongan': Lowongan,
    '/berita': Berita,
    '/berita/:slug': BeritaDetail,
    '*': NotFound
  };
</script>

<div class="min-h-screen flex flex-col bg-krem-50">
  <Navbar />
  <main class="flex-1">
    <Router {routes} />
  </main>
  <Footer />
</div>
