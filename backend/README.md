# gerbosari-backend

Backend API for the Desa Gerbosari village profile site. Serves dynamic features (galeri, data penduduk, lowongan, berita) over JSON to the Svelte SPA. Rust + Axum + sqlx (MySQL, async).

## Prerequisites

- Rust stable (toolchain file pins `stable`)
- MySQL 8.x running locally or reachable from the dev machine
- `sqlx-cli` is optional; migrations run automatically at startup

## Setup

```bash
cp .env.example .env
# edit .env — at minimum set DATABASE_URL
mysql -u root -p -e "CREATE DATABASE gerbosari CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
cargo build
cargo run
```

On startup the process loads `.env`, opens the MySQL pool, applies any pending migrations under `./migrations`, then binds to `BIND_ADDR` (default `0.0.0.0:3000`). Health check:

```bash
curl http://localhost:3000/api/health
```

## Environment

| Variable       | Required | Default                  | Notes                                  |
|----------------|----------|--------------------------|----------------------------------------|
| `DATABASE_URL` | yes      | —                        | `mysql://user:pass@host:port/db`       |
| `BIND_ADDR`    | no       | `0.0.0.0:3000`           | host:port the server listens on        |
| `CORS_ORIGIN`  | no       | `http://localhost:5173`  | comma-separated list of allowed origins|
| `RUST_LOG`     | no       | `info,sqlx=warn`         | standard `tracing-subscriber` filter   |
| `APP_ENV`      | no       | `development`            | `production` switches logs to JSON     |

## API Surface

All endpoints under `/api`, JSON only, GET only (read API).

| Method | Path                       | Notes                                   |
|--------|----------------------------|-----------------------------------------|
| GET    | `/api/health`              | Liveness probe                          |
| GET    | `/api/galeri`              | Optional `?kategori=kegiatan\|wisata\|budaya\|agrowisata` |
| GET    | `/api/galeri/:id`          | UUID                                    |
| GET    | `/api/penduduk/ringkasan`  | Aggregate totals + per pedukuhan        |
| GET    | `/api/lowongan`            | Optional `?kategori=umkm\|formal\|freelance&status=aktif\|tutup\|arsip` |
| GET    | `/api/lowongan/:id`        | UUID                                    |
| GET    | `/api/berita`              | Optional `?kategori=berita\|agenda` (list view, no `konten`) |
| GET    | `/api/berita/:slug`        | Full body including `konten` (markdown) |

Error shape:

```json
{ "error": "not_found", "message": "Resource not found" }
```

## Architecture

Clean-architecture layering. Inner layers never depend on outer ones.

```
src/
  domain/         pure entities, enums, repository traits
  application/    use-case services, generic over repository traits
  infrastructure/ MySQL pool + concrete sqlx-backed repositories
  presentation/   Axum router, handlers, DTOs, AppState
  config.rs       env-driven AppConfig
  error.rs        AppError + IntoResponse
  main.rs         bootstrap (env, tracing, pool, migrations, server)
```

- `domain` has zero `sqlx`/`axum`/`serde-derive` on entities; DTOs in `presentation/dto` handle serialization.
- Repositories are async traits (`async_trait`); services hold `Arc<dyn Trait>` so swapping persistence (e.g. for tests) is trivial.
- Errors flow through `AppError`; internal DB/anyhow errors are logged via `tracing::error!` and surfaced to clients as a generic 500.

## Adding a new endpoint

1. Add the entity and any enums under `src/domain/<feature>.rs`.
2. Extend `src/domain/repository.rs` with the trait the feature needs.
3. Implement the trait in `src/infrastructure/persistence/<feature>_repo.rs`.
4. Create a service in `src/application/<feature>_service.rs` wrapping the repo.
5. Add request/response shapes in `src/presentation/dto/<feature>_dto.rs`.
6. Write the handler in `src/presentation/handlers/<feature>.rs`.
7. Wire the route into `src/presentation/router.rs` and register the service on `AppState` (`src/presentation/state.rs`, constructed in `main.rs`).

## Migrations

Schema lives in `./migrations` (owned by the database track). `sqlx::migrate!` runs them at startup; do not commit ad-hoc DDL into the backend code.
