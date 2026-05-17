# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

Final project for "Komputasi Awan" (Cloud Computing) course at UIN Sunan Kalijaga, semester 6. The deliverable is a cloud-hosted village profile website for **Desa Gerbosari, Kulon Progo, DIY**. Hosting target is AWS (instance provisioned under a shared lecturer account "addin").

The repository currently contains only source materials — no code has been scaffolded yet:
- `Desa Gerbosari.docx` / `desa_gerbosari.docx` — primary content source (history, vision/mission, organizational structure, population data, embedded photos for the gallery)
- `PEMBUATAN-PROFIL-DESA-GERBOSARI.pdf` — secondary content reference
- `instruksi.md` — assignment brief from the lecturer

When scaffolding, extract images embedded in the `.docx` files (unzip the docx and pull from `word/media/`) and split them into a dedicated asset folder for the gallery.

## Architecture (Required by Assignment)

The site must be split into two cooperating layers — this split is **graded**, not a stylistic choice:

1. **Static frontend — Svelte + Vite.** Hosts the always-on profile content: sejarah desa, visi misi, struktur organisasi, peta wilayah. Source content comes from the `.docx`/`.pdf` files in this directory plus `gerbosari-kulonprogo.desa.id` and Wikipedia. The map should use an embedded provider (Leaflet/OSM or Google Maps embed) — coordinates and boundary info are in the source docs.

2. **Dynamic backend — Rust + Axum + sqlx + MySQL.** Serves the dynamic features over JSON API: photo gallery (`galeri`), population summary (`data penduduk` per pedukuhan), job postings (`lowongan` — UMKM/formal/freelance), and news/agenda (`berita`). Async only — no sync/legacy crates. The frontend consumes this over HTTP; do not server-render dynamic content from Rust.

The static and dynamic layers are deployed independently on AWS. Treat them as two separate deployables from day one (separate folders, separate build pipelines) — do not merge them into a single monorepo build step.

## Conventions

- Source content is in Indonesian; UI copy must be Indonesian. Code identifiers, comments, and commit messages stay in English.
- Cloud target is AWS per `instruksi.md`. Do not silently swap to Vercel/Netlify/etc. — that would violate the "shared AWS instance under addin's account" constraint.
- Four dynamic features in scope: **galeri foto**, **data penduduk**, **lowongan kerja (UMKM/lokal)**, and **berita/agenda**. All four must be CRUD-capable from the backend.
- No emoji anywhere in the UI. Use inline SVG (Lucide-style stroke icons) or `<svg>` files — never Unicode emoji glyphs in JSX/Svelte templates or content JSON.
- Design must reflect the village's character (Menoreh hills, agrowisata teh/kopi/durian, batik & topeng kerajinan, ekonomi kreatif) — not a generic SaaS template. Earthy palette (deep teal-green, terracotta, warm cream), serif accent for sejarah/legenda headings, sans for body.

## Repository Layout

```
tugas-akhir/
├── content/          # Source-of-truth content extracted from docx/pdf (JSON + images). Owned by content pipeline; both frontend & backend seed from here.
├── frontend/         # Svelte 4 + Vite SPA. Clean separation: lib/components, lib/api, routes, stores.
├── backend/          # Rust + Axum + sqlx. Clean architecture: domain / application / infrastructure / presentation.
├── deploy/           # docker-compose, nginx, AWS notes.
└── _extract/         # Raw docx unzip output. Throwaway — do not commit, do not edit.
```

`content/` is the bridge between the two deployables. The frontend imports static content JSON at build time; the backend's seed migrations read the same JSON to populate `lowongan`/`berita`/`galeri` initial rows. Keep it canonical — do not duplicate village text into either frontend or backend source.
