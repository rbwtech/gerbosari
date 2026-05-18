<script lang="ts">
  import { onDestroy } from 'svelte';
  import { fly } from 'svelte/transition';
  import { location, navigate } from '../../lib/router';
  import { authState, clearAuth } from '../../lib/auth';
  import { LogOut, Menu, X } from '../../lib/components/icons';
  import AdminNav from './AdminNav.svelte';

  /** Optional page title overriding the route-derived breadcrumb. */
  export let title: string | undefined = undefined;

  // Mapping from route path -> human-readable breadcrumb segment. Keeps the
  // labels consistent across the dashboard, sidebar, and top bar.
  const SEGMENT_LABELS: Record<string, string> = {
    admin: 'Admin',
    galeri: 'Galeri',
    lowongan: 'Lowongan',
    berita: 'Berita',
    penduduk: 'Data Penduduk',
    new: 'Tambah Baru'
  };

  const UUID_RE = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

  function humanise(segment: string): string {
    const direct = SEGMENT_LABELS[segment];
    if (direct) return direct;
    if (UUID_RE.test(segment)) return `#${segment.slice(0, 8)}`;
    // Slug → "pembangunan-jalan-2025" becomes "Pembangunan Jalan 2025". Keeps
    // the breadcrumb / page title legible without depending on the row's
    // judul being passed in via prop.
    return segment
      .split('-')
      .map((w) => (w.length === 0 ? w : w.charAt(0).toUpperCase() + w.slice(1)))
      .join(' ');
  }

  $: segments = $location
    .split('/')
    .filter((s) => s.length > 0)
    .map((segment, index, all) => {
      const href = '/' + all.slice(0, index + 1).join('/');
      const isLast = index === all.length - 1;
      // Caller-supplied `title` (e.g. an entity's loaded judul) overrides the
      // deepest segment so opaque UUIDs / freshly-bootstrapped rows still
      // render a human label in the breadcrumb.
      const label = isLast && title ? title : humanise(segment);
      return { label, href };
    });

  // Top-bar title: explicit prop wins, otherwise fall back to the deepest crumb.
  $: pageTitle = title ?? (segments.length > 0 ? segments[segments.length - 1].label : 'Admin');

  // Mobile drawer state. Mirrors the public Navbar pattern: sliding panel from
  // the left with a backdrop, body scroll lock while open, auto-close on route
  // change so navigation never strands the drawer in the open state.
  let drawerOpen = false;
  let previousLocation = $location;

  function closeDrawer(): void {
    drawerOpen = false;
  }

  function toggleDrawer(): void {
    drawerOpen = !drawerOpen;
  }

  // Reactive scroll lock: avoids the document jumping around when the drawer
  // animates in. Cleaned up explicitly in onDestroy so SPA navigation never
  // leaves the body permanently locked.
  $: if (typeof document !== 'undefined') {
    document.body.style.overflow = drawerOpen ? 'hidden' : '';
  }

  // Route change closes the drawer. Tracking the previous value avoids
  // closing on the very first reactive run before the user opens it.
  $: if ($location !== previousLocation) {
    previousLocation = $location;
    drawerOpen = false;
  }

  onDestroy(() => {
    if (typeof document !== 'undefined') document.body.style.overflow = '';
  });

  function handleLogout(): void {
    clearAuth();
    navigate('/admin/login');
  }
</script>

<div class="min-h-screen bg-krem-50">
  <!-- Desktop sidebar: pinned to the viewport, not in flow. Using `fixed`
       (instead of `sticky top-0 h-screen`) keeps the dark bg covering the
       full left rail even when page content scrolls past one viewport — a
       sticky h-screen sidebar visually "ends" once the user scrolls past
       its container, exposing the page bg behind it. -->
  <aside
    class="hidden md:flex fixed left-0 top-0 z-30 h-screen w-60 bg-arang-900 text-krem-100
           border-r border-arang-800 flex-col"
    aria-label="Sidebar admin"
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
      <div class="flex flex-col leading-tight">
        <span class="font-serif text-sm font-semibold text-white">Gerbosari</span>
        <span class="text-[10px] font-medium uppercase tracking-widest text-krem-300">Panel Admin</span>
      </div>
    </a>

    <div class="flex-1 overflow-y-auto py-4">
      <AdminNav />
    </div>

    <div class="border-t border-arang-800 p-3">
      <button
        type="button"
        class="w-full flex items-center gap-3 rounded-md px-3 min-h-11 text-sm font-medium
               text-krem-200 hover:bg-arang-800 hover:text-white
               transition-colors duration-200 ease-out"
        on:click={handleLogout}
        title="Keluar"
      >
        <LogOut class="w-[18px] h-[18px] shrink-0" strokeWidth={1.75} aria-hidden="true" />
        <span>Keluar</span>
      </button>
    </div>
  </aside>

  <!-- Content column. md:pl-60 reserves the sidebar's footprint since the
       sidebar itself is now `fixed` (out-of-flow). -->
  <div class="min-w-0 flex flex-col md:pl-60">
    <header
      class="sticky top-0 z-30 h-14 md:h-16 bg-white border-b border-krem-200 flex items-center"
    >
      <div class="flex-1 min-w-0 flex items-center justify-between px-4 md:px-6 gap-3 md:gap-4">
        <div class="flex items-center gap-2 min-w-0 flex-1">
          <!-- Hamburger: mobile only, top-left of top bar. -->
          <button
            type="button"
            class="md:hidden inline-flex items-center justify-center min-h-11 min-w-11 -ml-2
                   rounded-md text-arang-900 hover:bg-krem-100
                   transition-colors duration-200 ease-out"
            aria-label={drawerOpen ? 'Tutup navigasi' : 'Buka navigasi'}
            aria-expanded={drawerOpen}
            aria-controls="admin-mobile-drawer"
            on:click={toggleDrawer}
          >
            {#if drawerOpen}
              <X class="w-5 h-5" strokeWidth={2} aria-hidden="true" />
            {:else}
              <Menu class="w-5 h-5" strokeWidth={2} aria-hidden="true" />
            {/if}
          </button>

          <div class="min-w-0 flex-1">
            <nav class="hidden md:flex items-center gap-1.5 text-xs text-arang-500" aria-label="Breadcrumb">
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
            <h1
              class="font-serif text-base md:text-xl font-semibold text-arang-900 truncate leading-tight"
            >
              {pageTitle}
            </h1>
          </div>
        </div>

        <div class="flex items-center gap-2 md:gap-3 shrink-0">
          {#if $authState.user}
            <span class="hidden sm:inline text-sm font-medium text-arang-900 truncate max-w-[160px]">
              {$authState.user.username}
            </span>
            <button
              type="button"
              class="inline-flex items-center justify-center min-h-11 min-w-11 rounded-md
                     text-arang-700 hover:bg-krem-100 hover:text-terakota-700
                     transition-colors duration-200 ease-out"
              on:click={handleLogout}
              aria-label="Keluar dari panel admin"
              title="Keluar"
            >
              <LogOut class="w-[18px] h-[18px]" strokeWidth={1.75} aria-hidden="true" />
            </button>
          {/if}
        </div>
      </div>
    </header>

    <main class="flex-1 p-4 md:p-6 lg:p-8">
      <slot />
    </main>
  </div>
</div>

<!-- Mobile drawer: portal-less overlay. Renders only when needed to keep the
     DOM lean on desktop. fly from -300 mirrors the public Navbar treatment but
     anchored on the left edge. -->
{#if drawerOpen}
  <div
    class="md:hidden fixed inset-0 z-50"
    role="dialog"
    aria-modal="true"
    aria-label="Navigasi admin seluler"
  >
    <button
      type="button"
      class="absolute inset-0 bg-arang-900/50"
      aria-label="Tutup navigasi"
      on:click={closeDrawer}
    ></button>

    <div
      id="admin-mobile-drawer"
      class="absolute left-0 top-0 h-full w-72 max-w-[85%] bg-arang-900 text-krem-100
             border-r border-arang-800 flex flex-col shadow-xl"
      transition:fly={{ x: -300, duration: 220 }}
    >
      <div class="flex items-center justify-between h-14 px-4 border-b border-arang-800">
        <a
          href="/admin"
          class="flex items-center gap-2.5"
          aria-label="Dashboard admin"
          on:click|preventDefault={() => {
            closeDrawer();
            navigate('/admin');
          }}
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
          <div class="flex flex-col leading-tight">
            <span class="font-serif text-sm font-semibold text-white">Gerbosari</span>
            <span class="text-[10px] font-medium uppercase tracking-widest text-krem-300">Panel Admin</span>
          </div>
        </a>
        <button
          type="button"
          class="inline-flex items-center justify-center min-h-11 min-w-11 rounded-md
                 text-krem-200 hover:bg-arang-800 hover:text-white"
          aria-label="Tutup navigasi"
          on:click={closeDrawer}
        >
          <X class="w-5 h-5" strokeWidth={2} aria-hidden="true" />
        </button>
      </div>

      <div class="flex-1 overflow-y-auto py-3">
        <AdminNav on:navigate={closeDrawer} />
      </div>

      <div class="border-t border-arang-800 p-3">
        <button
          type="button"
          class="w-full flex items-center gap-3 rounded-md px-3 min-h-11 text-sm font-medium
                 text-krem-200 hover:bg-arang-800 hover:text-white
                 transition-colors duration-200 ease-out"
          on:click={handleLogout}
        >
          <LogOut class="w-[18px] h-[18px] shrink-0" strokeWidth={1.75} aria-hidden="true" />
          <span>Keluar</span>
        </button>
      </div>
    </div>
  </div>
{/if}
