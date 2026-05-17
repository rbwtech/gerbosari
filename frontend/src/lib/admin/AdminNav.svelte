<script lang="ts">
  import type { ComponentType, SvelteComponent } from 'svelte';
  import { link, location } from '../../lib/router';
  import {
    LayoutDashboard,
    ImageIcon,
    Briefcase,
    Newspaper,
    Users
  } from '../../lib/components/icons';

  /** When true, render the icon-only collapsed treatment used at <lg breakpoints. */
  export let collapsed = false;

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

  // Active-link logic: exact match for the dashboard root, prefix match for the
  // CRUD sections so list/detail/edit views all keep their nav entry highlighted.
  function isActive(href: string, current: string): boolean {
    if (href === '/admin') return current === '/admin';
    return current === href || current.startsWith(`${href}/`);
  }
</script>

<nav class="flex flex-col gap-1 px-3" aria-label="Navigasi admin">
  {#each items as item}
    {@const active = isActive(item.href, $location)}
    <a
      href={item.href}
      use:link
      class="group flex items-center gap-3 rounded-md px-3 py-2.5 text-sm font-medium
             transition-colors duration-200 ease-out
             {active
               ? 'bg-arang-800 text-white'
               : 'text-krem-200 hover:bg-arang-800 hover:text-white'}"
      aria-current={active ? 'page' : undefined}
      title={collapsed ? item.label : undefined}
    >
      <svelte:component
        this={item.icon}
        class="w-[18px] h-[18px] shrink-0 {active ? 'text-white' : 'text-krem-300 group-hover:text-white'}"
        strokeWidth={1.75}
        aria-hidden="true"
      />
      {#if !collapsed}
        <span class="truncate">{item.label}</span>
      {/if}
    </a>
  {/each}
</nav>
