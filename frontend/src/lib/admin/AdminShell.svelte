<script lang="ts">
  import { location, navigate } from '../../lib/router';
  import { authState, clearAuth } from '../../lib/auth';
  import { LogOut } from '../../lib/components/icons';
  import AdminNav from './AdminNav.svelte';

  /** Optional page title overriding the route-derived breadcrumb. */
  export let title: string | undefined = undefined;

  // Mapping from route path → human-readable breadcrumb segment. Keeps the
  // labels consistent across the dashboard, sidebar, and top bar.
  const SEGMENT_LABELS: Record<string, string> = {
    admin: 'Admin',
    galeri: 'Galeri',
    lowongan: 'Lowongan',
    berita: 'Berita',
    penduduk: 'Data Penduduk',
    new: 'Tambah Baru'
  };

  function humanise(segment: string): string {
    const direct = SEGMENT_LABELS[segment];
    if (direct) return direct;
    // UUID / numeric IDs: keep short, prefix with hash so it reads as an identifier.
    if (segment.length > 12) return `#${segment.slice(0, 8)}`;
    return segment.charAt(0).toUpperCase() + segment.slice(1);
  }

  $: segments = $location
    .split('/')
    .filter((s) => s.length > 0)
    .map((segment, index, all) => {
      const href = '/' + all.slice(0, index + 1).join('/');
      return { label: humanise(segment), href };
    });

  // Top-bar title: explicit prop wins, otherwise fall back to the deepest crumb.
  $: pageTitle = title ?? (segments.length > 0 ? segments[segments.length - 1].label : 'Admin');

  function handleLogout(): void {
    clearAuth();
    navigate('/admin/login');
  }
</script>

<div class="min-h-screen flex bg-krem-50">
  <!-- Sidebar. Collapses to icon-only below the lg breakpoint to preserve content width on mobile. -->
  <aside
    class="sticky top-0 h-screen w-16 lg:w-60 shrink-0 bg-arang-900 text-krem-100
           border-r border-arang-800 flex flex-col"
  >
    <a
      href="/admin"
      class="flex items-center gap-2.5 h-16 px-4 border-b border-arang-800 hover:bg-arang-800/60
             transition-colors duration-200 ease-out"
      aria-label="Dashboard admin"
      on:click|preventDefault={() => navigate('/admin')}
    >
      <svg
        viewBox="0 0 32 32"
        class="w-7 h-7 shrink-0"
        fill="none"
        stroke="currentColor"
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="22" cy="9" r="2.25" class="text-terakota-400" fill="currentColor" stroke="none" />
        <path d="M3 25 L11 13 L17 20 L23 11 L29 25 Z" class="text-menoreh-300" />
        <path d="M3 25 L29 25" class="text-menoreh-300" />
      </svg>
      <div class="hidden lg:flex flex-col leading-tight">
        <span class="font-serif text-sm font-semibold text-white">Gerbosari</span>
        <span class="text-[10px] font-medium uppercase tracking-widest text-krem-300">Panel Admin</span>
      </div>
    </a>

    <div class="flex-1 overflow-y-auto py-4">
      <!-- Mobile shows the icon-only nav, lg+ shows the full label nav. Two
           passes is cheaper than runtime media-query JS for a five-row menu. -->
      <div class="lg:hidden">
        <AdminNav collapsed />
      </div>
      <div class="hidden lg:block">
        <AdminNav />
      </div>
    </div>

    <div class="border-t border-arang-800 p-3">
      <button
        type="button"
        class="w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm font-medium
               text-krem-200 hover:bg-arang-800 hover:text-white
               transition-colors duration-200 ease-out"
        on:click={handleLogout}
        title="Keluar"
      >
        <LogOut class="w-[18px] h-[18px] shrink-0" strokeWidth={1.75} aria-hidden="true" />
        <span class="hidden lg:inline">Keluar</span>
      </button>
    </div>
  </aside>

  <!-- Content column -->
  <div class="flex-1 min-w-0 flex flex-col">
    <header class="sticky top-0 z-30 h-16 bg-white border-b border-krem-200 flex items-center">
      <div class="flex-1 min-w-0 flex items-center justify-between px-6 gap-4">
        <div class="min-w-0">
          <nav class="hidden sm:flex items-center gap-1.5 text-xs text-arang-500" aria-label="Breadcrumb">
            {#each segments as crumb, i}
              {#if i > 0}
                <span aria-hidden="true" class="text-arang-300">/</span>
              {/if}
              {#if i === segments.length - 1}
                <span class="text-arang-700 font-medium truncate" aria-current="page">{crumb.label}</span>
              {:else}
                <a
                  href={crumb.href}
                  on:click|preventDefault={() => navigate(crumb.href)}
                  class="hover:text-menoreh-700 transition-colors duration-200 ease-out truncate"
                >
                  {crumb.label}
                </a>
              {/if}
            {/each}
          </nav>
          <h1 class="font-serif text-lg sm:text-xl font-semibold text-arang-900 truncate leading-tight">
            {pageTitle}
          </h1>
        </div>

        <div class="flex items-center gap-3 shrink-0">
          {#if $authState.user}
            <div class="hidden sm:flex flex-col items-end leading-tight">
              <span class="text-sm font-medium text-arang-900">{$authState.user.username}</span>
              <span class="text-[10px] font-medium uppercase tracking-widest text-arang-500">Administrator</span>
            </div>
            <div
              class="w-9 h-9 rounded-full bg-menoreh-700 text-white flex items-center justify-center
                     text-sm font-semibold select-none"
              aria-hidden="true"
            >
              {$authState.user.username.slice(0, 1).toUpperCase()}
            </div>
          {/if}
        </div>
      </div>
    </header>

    <main class="flex-1 p-6 lg:p-8">
      <slot />
    </main>
  </div>
</div>
