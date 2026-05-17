# Deploy

Operational artifacts for hosting Desa Gerbosari at `gerbosari.rbwtech.io`
on an aaPanel-managed cloud VM. No Docker, no containers — the Rust binary
runs as a systemd service and nginx (managed by aaPanel) reverse-proxies it.

| File | Purpose |
| --- | --- |
| `aapanel-setup.md` | One-time provisioning + per-deploy runbook. Start here. |
| `nginx.conf` | aaPanel vhost: serves the frontend `dist/` at the site root, proxies `/api/` to the Rust backend on 127.0.0.1:3000, aliases `/images/`, denies `/_backend/`. |
| `gerbosari-backend.service` | Systemd unit running the Rust backend as `www`, with `WorkingDirectory=/www/wwwroot/gerbosari.rbwtech.io/_backend`. |
| `.env.production.example` | Template for `_backend/.env`. Install with mode `0600`, owner `www`. Never commit the populated file. |
| `aws-setup.md` | Stub pointing at `aapanel-setup.md`. |

Database migrations live in `../backend/migrations/` and are applied automatically
on backend startup via `sqlx::migrate!("./migrations")` — there is no manual
migration step in the deploy flow.
