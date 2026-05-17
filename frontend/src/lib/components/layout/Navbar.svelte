<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fly } from 'svelte/transition';
  import { location, link } from 'svelte-spa-router';
  import { Menu, X } from '../icons';

  interface NavItem {
    href: string;
    label: string;
  }

  const items: NavItem[] = [
    { href: '/', label: 'Beranda' },
    { href: '/sejarah', label: 'Sejarah' },
    { href: '/visi-misi', label: 'Visi & Misi' },
    { href: '/struktur-organisasi', label: 'Struktur' },
    { href: '/peta-wilayah', label: 'Peta' },
    { href: '/galeri', label: 'Galeri' },
    { href: '/data-penduduk', label: 'Data Penduduk' },
    { href: '/lowongan', label: 'Lowongan' },
    { href: '/berita', label: 'Berita' }
  ];

  let mobileOpen = false;
  let scrolled = false;

  function isActive(href: string, current: string): boolean {
    if (href === '/') return current === '/' || current === '';
    return current === href || current.startsWith(`${href}/`);
  }

  function closeMobile() {
    mobileOpen = false;
  }

  // Lightweight scroll listener with rAF coalescing — toggles the elevated
  // navbar treatment once the user scrolls past the 8px threshold.
  let ticking = false;
  function onScroll() {
    if (ticking) return;
    ticking = true;
    requestAnimationFrame(() => {
      scrolled = window.scrollY > 8;
      ticking = false;
    });
  }

  onMount(() => {
    scrolled = window.scrollY > 8;
    window.addEventListener('scroll', onScroll, { passive: true });
  });

  onDestroy(() => {
    if (typeof window !== 'undefined') {
      window.removeEventListener('scroll', onScroll);
    }
  });

  $: $location, closeMobile();
</script>

<header
  class="sticky top-0 z-40 transition-colors duration-200 ease-out
         {scrolled
           ? 'bg-white/95 backdrop-blur-sm border-b border-krem-200 shadow-sm'
           : 'bg-transparent border-b border-transparent'}"
>
  <div class="container-page flex h-16 items-center justify-between">
    <a
      href="/"
      use:link
      class="flex items-center gap-2.5 group"
      on:click={closeMobile}
      aria-label="Beranda Desa Gerbosari"
    >
      <!-- Stylized hill silhouette + sun dot — references the Menoreh range -->
      <svg
        viewBox="0 0 32 32"
        class="w-8 h-8"
        fill="none"
        stroke="currentColor"
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="22" cy="9" r="2.25" class="text-terakota-500" fill="currentColor" stroke="none" />
        <path d="M3 25 L11 13 L17 20 L23 11 L29 25 Z" class="text-menoreh-700" />
        <path d="M3 25 L29 25" class="text-menoreh-700" />
      </svg>
      <span class="font-serif text-lg font-semibold text-arang-900 tracking-tight">
        Desa Gerbosari
      </span>
    </a>

    <nav class="hidden lg:flex items-center gap-1" aria-label="Navigasi utama">
      {#each items as item}
        {@const active = isActive(item.href, $location)}
        <a
          href={item.href}
          use:link
          class="relative px-3 py-2 text-xs font-semibold uppercase tracking-widest
                 transition-colors duration-200 ease-out
                 {active
                   ? 'text-menoreh-800 underline decoration-2 decoration-menoreh-700 underline-offset-8'
                   : 'text-arang-700 hover:text-menoreh-700'}"
          aria-current={active ? 'page' : undefined}
        >
          {item.label}
        </a>
      {/each}
    </nav>

    <button
      class="lg:hidden inline-flex h-10 w-10 items-center justify-center rounded-md
             text-arang-900 hover:bg-krem-100 transition-colors duration-200 ease-out"
      aria-label={mobileOpen ? 'Tutup menu' : 'Buka menu'}
      aria-expanded={mobileOpen}
      aria-controls="mobile-nav-panel"
      on:click={() => (mobileOpen = !mobileOpen)}
    >
      {#if mobileOpen}
        <X class="w-5 h-5" strokeWidth={2} />
      {:else}
        <Menu class="w-5 h-5" strokeWidth={2} />
      {/if}
    </button>
  </div>

  {#if mobileOpen}
    <!-- Sliding panel from the right; backdrop closes it on click -->
    <div
      class="lg:hidden fixed inset-0 z-50"
      role="dialog"
      aria-modal="true"
      aria-label="Menu seluler"
    >
      <button
        type="button"
        class="absolute inset-0 bg-arang-900/40"
        aria-label="Tutup menu"
        on:click={closeMobile}
      ></button>

      <div
        id="mobile-nav-panel"
        class="absolute right-0 top-0 h-full w-72 max-w-[85%] bg-white border-l border-krem-200 shadow-sm flex flex-col"
        transition:fly={{ x: 320, duration: 220 }}
      >
        <div class="flex items-center justify-between h-16 px-5 border-b border-krem-200">
          <span class="font-serif text-base font-semibold text-arang-900">Menu</span>
          <button
            type="button"
            class="inline-flex h-9 w-9 items-center justify-center rounded-md text-arang-700 hover:bg-krem-100"
            aria-label="Tutup menu"
            on:click={closeMobile}
          >
            <X class="w-5 h-5" strokeWidth={2} />
          </button>
        </div>
        <nav class="flex flex-col px-2 py-3" aria-label="Navigasi seluler">
          {#each items as item}
            {@const active = isActive(item.href, $location)}
            <a
              href={item.href}
              use:link
              on:click={closeMobile}
              class="px-3 py-3 rounded-md text-sm font-medium transition-colors duration-200 ease-out
                     {active
                       ? 'bg-menoreh-50 text-menoreh-800'
                       : 'text-arang-700 hover:bg-krem-100'}"
              aria-current={active ? 'page' : undefined}
            >
              {item.label}
            </a>
          {/each}
        </nav>
      </div>
    </div>
  {/if}
</header>
