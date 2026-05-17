<script lang="ts">
  import { Router } from './lib/router';
  import Navbar from './lib/components/layout/Navbar.svelte';
  import Footer from './lib/components/layout/Footer.svelte';

  import { location } from './lib/router';

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

  // Admin shell + auth + CRUD pages.
  import AdminLogin from './routes/admin/Login.svelte';
  import AdminDashboard from './routes/admin/Dashboard.svelte';
  import AdminGaleriList from './routes/admin/galeri/List.svelte';
  import AdminGaleriEdit from './routes/admin/galeri/Edit.svelte';
  import AdminLowonganList from './routes/admin/lowongan/List.svelte';
  import AdminLowonganEdit from './routes/admin/lowongan/Edit.svelte';
  import AdminBeritaList from './routes/admin/berita/List.svelte';
  import AdminBeritaEdit from './routes/admin/berita/Edit.svelte';
  import AdminPendudukList from './routes/admin/penduduk/List.svelte';
  import AdminPendudukEdit from './routes/admin/penduduk/Edit.svelte';

  const routes = {
    // Public site
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

    // Admin
    '/admin': AdminDashboard,
    '/admin/login': AdminLogin,
    '/admin/galeri': AdminGaleriList,
    '/admin/galeri/new': AdminGaleriEdit,
    '/admin/galeri/:id': AdminGaleriEdit,
    '/admin/lowongan': AdminLowonganList,
    '/admin/lowongan/new': AdminLowonganEdit,
    '/admin/lowongan/:id': AdminLowonganEdit,
    '/admin/berita': AdminBeritaList,
    '/admin/berita/new': AdminBeritaEdit,
    '/admin/berita/:slug': AdminBeritaEdit,
    '/admin/penduduk': AdminPendudukList,
    '/admin/penduduk/:pedukuhan': AdminPendudukEdit,

    '*': NotFound
  };

  // The admin section renders its own chrome (AdminShell) and full-screen
  // login layout, so the public Navbar / Footer must stand down on /admin/*.
  $: isAdminRoute = $location === '/admin' || $location.startsWith('/admin/');
</script>

{#if isAdminRoute}
  <Router {routes} />
{:else}
  <div class="min-h-screen flex flex-col bg-krem-50">
    <Navbar />
    <main class="flex-1">
      <Router {routes} />
    </main>
    <Footer />
  </div>
{/if}
