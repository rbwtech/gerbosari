# Gerbosari Frontend

SPA profil Desa Gerbosari. Vanilla Svelte 4 + Vite 5 + TypeScript, hash routing via `svelte-spa-router`, styled with Tailwind 3.

## Stack

- Svelte 4 (bukan SvelteKit)
- Vite 5 + TypeScript
- Tailwind CSS 3 dengan tema khas Menoreh (menoreh / terakota / krem / arang)
- Lucide icons (`lucide-svelte`)
- Leaflet + OpenStreetMap untuk peta
- `marked` untuk render konten markdown berita

## Setup

```bash
cp .env.example .env.local
# sesuaikan VITE_API_BASE bila backend tidak berjalan di localhost:3000
npm install
npm run dev          # dev server di http://localhost:5173
npm run build        # output ke dist/
npm run preview      # uji build production lokal
npm run check        # svelte-check
```

## Backend Endpoint

Set `VITE_API_BASE` di `.env.local` (default `http://localhost:3000/api`). Endpoint yang dikonsumsi: `GET /galeri`, `GET /penduduk/ringkasan`, `GET /lowongan`, `GET /lowongan/:id`, `GET /berita`, `GET /berita/:slug`.

## Struktur Folder

- `src/routes/` — satu file Svelte per halaman, dirangkai di `App.svelte`.
- `src/lib/api/` — wrapper `fetch` typed per resource backend.
- `src/lib/content/` — modul typed yang mengimpor JSON statis dari `../../../content/pages/` (build-time).
- `src/lib/components/` — `layout/` (Navbar, Footer, PageHeader), `ui/` (Button, Card, Badge, Skeleton, EmptyState), `icons/` (barrel lucide).
- `src/lib/types/` — DTO yang mengikuti respons backend.
- `src/lib/stores/` — store kecil (mis. toast).

Konten statis untuk Beranda, Sejarah, Visi & Misi, Struktur Organisasi, dan Peta Wilayah dibaca dari `content/pages/*.json` di root repo (di-maintain oleh pipeline konten terpisah).
