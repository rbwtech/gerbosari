# Desa Gerbosari — Profil Desa Berbasis Cloud

Cloud-hosted village profile site for **Desa Gerbosari, Kecamatan Samigaluh, Kabupaten Kulon Progo, DIY**. Built as the final project for the *Komputasi Awan* (Cloud Computing) course at UIN Sunan Kalijaga, semester 6.

Live: <https://gerbosari.rbwtech.io>

## Why two deployables

The brief grades the split between **static profile content** and **dynamic CRUD features** — they are deployed as independent layers, not bundled into a single monolith:

```
                +-------------------+         +--------------------+
  Browser ----> | nginx (aaPanel)   | --------| Frontend (static)  |
                |   gerbosari.      |         | Svelte + Vite SPA  |
                |   rbwtech.io      |         +--------------------+
                |                   |
                |  /api/* proxy --> | --------> Backend (dynamic)
                +-------------------+          Rust + Axum + sqlx
                                               127.0.0.1:3000
                                                      |
                                                      v
                                               MySQL 8 (local)
```

Static pages (sejarah, visi-misi, struktur organisasi, peta wilayah) are pre-rendered into the SPA bundle from `content/pages/*.json`. Dynamic features (galeri, data penduduk, lowongan, berita & agenda) hit the Axum API and round-trip through MySQL.

## Stack

| Layer | Tech | Why |
|---|---|---|
| Frontend | Svelte 4 + Vite 5 + TypeScript + Tailwind 3 | SPA on a static host. No SvelteKit — keeps the build artifact a plain `dist/` folder that any CDN or nginx instance can serve. |
| Routing  | svelte-spa-router | Hash-based, zero server config. |
| Maps     | Leaflet + OpenStreetMap | No API key, no quota. |
| Backend  | Rust 1.x stable + Axum 0.7 + sqlx 0.8 (mysql, runtime-tokio-rustls) + tokio | Async-only. Clean-arch layers: `domain → application → infrastructure → presentation`. |
| Database | MySQL 8 | Managed by aaPanel on the same host. |
| Web      | nginx (aaPanel-managed) | Reverse-proxies `/api/*`, aliases `/images/`, serves the SPA. |
| Service  | systemd | `gerbosari-backend.service` runs the release binary as user `www`. |

Design palette is intentionally not generic: `menoreh` (deep teal-green forest), `terakota` (warm batik red), `krem` (paper cream), `tanah` (umber for historical sections), `arang` (warm slate text). Lora serif for sejarah/legenda; Inter for UI.

## Repository layout

```
tugas-akhir/
├── content/                  # Single source of truth for village content
│   ├── pages/*.json          # sejarah, visi-misi, struktur-organisasi, peta-wilayah, beranda
│   ├── seed/*.json           # galeri, lowongan, berita, penduduk-ringkasan
│   └── images/               # Renamed from .docx extracts (cover/, gallery/, peta/)
├── frontend/                 # Svelte 4 SPA — deploys to webroot
│   └── src/
│       ├── lib/api/          # Backend client (typed fetch wrappers)
│       ├── lib/components/   # Layout + UI primitives (Stat, Chip, Tabs, ...)
│       ├── lib/content/      # Static-content loaders (import build-time JSON)
│       ├── lib/types/        # DTO interfaces mirroring backend response shape
│       └── routes/           # One Svelte page per route, hash-based
├── backend/                  # Rust + Axum + sqlx — deploys to _backend/
│   ├── migrations/           # SQL files applied at startup via sqlx::migrate!
│   └── src/
│       ├── domain/           # Entities + repository traits (pure, no I/O)
│       ├── application/      # Use cases (services generic over repo trait)
│       ├── infrastructure/   # sqlx repository impls + connection pool
│       └── presentation/     # Axum router, handlers, DTOs (serde)
└── deploy/                   # Production artifacts (see deploy/README.md)
```

`content/` is canonical. The frontend imports `pages/*.json` at build time; the backend's seed migration mirrors the same row identities. Do not duplicate village text into either side.

## Quickstart — local development

Requires Node 20+, Rust stable, MySQL 8.

**Backend:**

```bash
cd backend
cp .env.example .env                # adjust DATABASE_URL if needed
mysql -uroot -p -e "CREATE DATABASE gerbosari CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
cargo run                            # migrations auto-apply on startup; binds 0.0.0.0:3000
```

**Frontend** (separate terminal):

```bash
cd frontend
cp .env.example .env.local           # VITE_API_BASE=http://localhost:3000/api
npm install
npm run dev                          # http://localhost:5173
```

Open <http://localhost:5173>. Nine routes are wired: `/`, `/sejarah`, `/visi-misi`, `/struktur-organisasi`, `/peta-wilayah`, `/galeri`, `/data-penduduk`, `/lowongan`, `/berita`, `/berita/:slug`.

## Production build

Both layers build into independent artifacts.

```bash
# Frontend → ./frontend/dist/  (index.html, assets/, favicon.svg)
cd frontend && npm ci && npm run build

# Backend → ./backend/target/release/gerbosari-backend  (single binary)
cd backend && cargo build --release
```

Vite reads `frontend/.env.production` during build, which pins `VITE_API_BASE=/api` so the SPA shares an origin with the API behind nginx.

## Deployment

Target host runs aaPanel; webroot is `/www/wwwroot/gerbosari.rbwtech.io/`. Full runbook in [`deploy/aapanel-setup.md`](deploy/aapanel-setup.md).

Final layout on the server:

```
/www/wwwroot/gerbosari.rbwtech.io/
├── index.html, assets/, favicon.svg   # frontend dist (publicly served)
├── content/images/                    # nginx alias /images/ -> here
└── _backend/                          # location ^~ /_backend/ deny all
    ├── gerbosari-backend              # systemd ExecStart
    ├── migrations/                    # bundled sqlx migrations
    └── .env                           # 0600, owner www
```

Migrations are bundled via `sqlx::migrate!("./migrations")` and apply automatically on backend startup — no manual `sqlx-cli migrate run` step in the deploy flow.

## Conventions

- Source content is in Indonesian. UI copy must be Indonesian. Code identifiers, comments, and commit messages stay in English.
- No emoji anywhere in the UI. Use inline SVG (Lucide-style stroke icons) or `<svg>` files. Audited via `grep`.
- API field names are snake_case to mirror Rust's serde output verbatim. The frontend's `lib/types/index.ts` is the contract document — change it only in lockstep with `backend/src/presentation/dto/*.rs`.
- Cards carry zero drop shadows by default. Subtle 1px borders + bg contrast only. No glassmorphism, no animated gradients (the single permitted gradient is the hero alpha overlay).
- Cloud target is AWS-class (aaPanel runs on any cloud VM — current: AWS EC2). Do not silently swap deploy targets.

## What would come next given more time

- **Admin CRUD UI.** Backend exposes read-only endpoints today; write methods exist in `domain::repository` traits but are not wired into handlers. An `/admin` SPA route + JWT-protected POST/PATCH/DELETE is the natural next step.
- **Real per-pedukuhan coordinates.** Current map uses a deterministic ring layout around the village office. Replace with surveyed lat/lon when available.
- **Image optimization pipeline.** Content images ship as-is from the source `.docx`. A `vite-plugin-imagetools` pass for `.webp` + `srcset` variants would cut LCP measurably.
- **OpenTelemetry.** Backend already wires `tracing`; an OTLP exporter to a hosted collector closes the observability loop.

## License & academic note

Educational project for the Komputasi Awan course, UIN Sunan Kalijaga. Content sourced from the `.docx` village profile (Devi & Hidayati, 2020), the Wikipedia entry on Gerbosari, and `gerbosari-kulonprogo.desa.id`. Source attribution preserved inside individual content files.
