<script lang="ts">
  import { createEventDispatcher, type ComponentType, type SvelteComponent } from 'svelte';
  import { link, location } from '../../lib/router';
  import {
    LayoutDashboard,
    ImageIcon,
    Briefcase,
    Newspaper,
    Users
  } from '../../lib/components/icons';

  interface AdminNavItem {
    href: string;
    label: string;
    icon: ComponentType<SvelteComponent>;
  }

  const items: AdminNavItem[] = [
    { href: '/admin', label: 'Dashboard', icon: LayoutDashboard },
    { href: '/admin/galeri', label: 'Galeri', icon: ImageIcon },
    { href: '/admin/lowongan', label: 'Lowongan', icon: Briefcase },
    { href: '/admin/berita', label: 'Berita', icon: Newspaper },
    { href: '/admin/penduduk', label: 'Data Penduduk', icon: Users }
  ];

  const dispatch = createEventDispatcher<{ navigate: void }>();

  // Active-link logic: exact match for the dashboard root, prefix match for the
  // CRUD sections so list/detail/edit views all keep their nav entry highlighted.
  function isActive(href: string, current: string): boolean {
    if (href === '/admin') return current === '/admin';
    return current === href || current.startsWith(`${href}/`);
  }
</script>

<nav class="flex flex-col gap-1 px-2" aria-label="Navigasi admin">
  {#each items as item}
    {@const active = isActive(item.href, $location)}
    <a
      href={item.href}
      use:link
      on:click={() => dispatch('navigate')}
      class="group relative flex items-center gap-3 rounded-md px-4 min-h-11 text-sm font-medium
             transition-colors duration-200 ease-out
             {active
               ? 'bg-arang-800 text-white'
               : 'text-krem-200 hover:bg-arang-800 hover:text-white'}"
      aria-current={active ? 'page' : undefined}
    >
      {#if active}
        <!-- Left accent bar: subtle visual anchor for the active nav row that
             survives both desktop sidebar and mobile drawer contexts. -->
        <span
          aria-hidden="true"
          class="absolute left-0 top-1.5 bottom-1.5 w-0.5 rounded-r bg-menoreh-400"
        ></span>
      {/if}
      <svelte:component
        this={item.icon}
        class="h-5 w-5 shrink-0 {active ? 'text-white' : 'text-krem-300 group-hover:text-white'}"
        strokeWidth={1.75}
        aria-hidden="true"
      />
      <span class="truncate">{item.label}</span>
    </a>
  {/each}
</nav>
