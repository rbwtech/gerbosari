# aaPanel Runbook — Desa Gerbosari

Target: any cloud VM (AWS EC2, DigitalOcean, etc.) running aaPanel on Ubuntu 22.04 / Debian 12.
Domain: `gerbosari.rbwtech.io`.
Webroot: `/www/wwwroot/gerbosari.rbwtech.io`.

## One-time provisioning

1. **In aaPanel UI:**
   - **Website** → Add Site → Domain `gerbosari.rbwtech.io`, Root `/www/wwwroot/gerbosari.rbwtech.io`, PHP **Pure static** (no PHP version), Database optional (we create one below).
   - **Database** → Add → name `gerbosari`, user `gerbosari`, password = strong (save it for `_backend/.env`).
   - **SSL** → Let's Encrypt → request cert for `gerbosari.rbwtech.io`. aaPanel will splice the `443` block into the site config and add an HTTP→HTTPS redirect on its own.

2. **Install build toolchains (SSH as root):**
   ```bash
   apt -y install build-essential pkg-config libssl-dev curl git
   # Rust (stable) under the www user so cargo can write target/ inside _backend
   sudo -u www -H bash -lc 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable'
   # Node 20 via nvm for the build user (your admin user, not www)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
   . ~/.nvm/nvm.sh && nvm install 20 && nvm use 20
   ```

3. **Create the backend tree inside the site root:**
   ```bash
   mkdir -p /www/wwwroot/gerbosari.rbwtech.io/_backend
   mkdir -p /www/wwwroot/gerbosari.rbwtech.io/content/images
   chown -R www:www /www/wwwroot/gerbosari.rbwtech.io
   ```

4. **Write the systemd unit** (the unit file in this repo points at the aaPanel paths already):
   ```bash
   cp deploy/gerbosari-backend.service /etc/systemd/system/
   systemctl daemon-reload
   ```

5. **Install the env file** (template in this repo):
   ```bash
   install -m 0600 -o www -g www \
       deploy/.env.production.example \
       /www/wwwroot/gerbosari.rbwtech.io/_backend/.env
   nano /www/wwwroot/gerbosari.rbwtech.io/_backend/.env   # fill DATABASE_URL password etc.
   ```

6. **Replace aaPanel's auto-generated site config** with the one in this repo (the original includes PHP enablement and lacks the API/SPA blocks):
   - Open aaPanel → Website → `gerbosari.rbwtech.io` → Config File.
   - Replace the contents with `deploy/nginx.conf` from this repo (keep the `#CERT-APPLY-CHECK--START..END` and `#REWRITE-START..END` include lines — aaPanel rewrites these automatically when SSL is renewed or rewrites are edited).
   - Save → aaPanel validates and reloads nginx.

## Per-deploy

Run from a checkout of the repo on the same machine (anywhere readable; the user below is your admin user, not `www`).

```bash
cd ~/gerbosari && git pull --ff-only

# --- Backend: build release binary, install into _backend, restart service ---
(cd backend && sudo -u www -H bash -lc 'cargo build --release')
install -m 0755 -o www -g www \
    backend/target/release/gerbosari-backend \
    /www/wwwroot/gerbosari.rbwtech.io/_backend/gerbosari-backend
# Migrations are bundled into the binary via sqlx::migrate!("./migrations"),
# but the runtime macro re-reads the folder so ship a copy alongside:
rsync -a --delete backend/migrations/ \
    /www/wwwroot/gerbosari.rbwtech.io/_backend/migrations/
chown -R www:www /www/wwwroot/gerbosari.rbwtech.io/_backend

systemctl restart gerbosari-backend
systemctl status gerbosari-backend --no-pager   # confirm active

# --- Frontend: build, deploy dist to site root ---
(cd frontend && npm ci && npm run build)
rsync -a --delete --exclude content frontend/dist/ \
    /www/wwwroot/gerbosari.rbwtech.io/

# --- Shared content images (served at /images/ via nginx alias) ---
rsync -a --delete content/images/ \
    /www/wwwroot/gerbosari.rbwtech.io/content/images/
chown -R www:www /www/wwwroot/gerbosari.rbwtech.io/content

# nginx reload only if you edited deploy/nginx.conf — aaPanel reloads itself
# whenever you save a site config. Manual:
nginx -t && systemctl reload nginx
```

## Verifying

```bash
curl https://gerbosari.rbwtech.io/healthz
curl https://gerbosari.rbwtech.io/api/galeri?kategori=wisata
curl https://gerbosari.rbwtech.io/api/penduduk/ringkasan
curl -I https://gerbosari.rbwtech.io/images/gallery/gerbosari-kegiatan-01.jpg
```

Open the site: <https://gerbosari.rbwtech.io>. All nine routes should load (Beranda, Sejarah, Visi & Misi, Struktur Organisasi, Peta Wilayah, Galeri, Data Penduduk, Lowongan, Berita & Agenda).

## Logs

```bash
journalctl -u gerbosari-backend -f
tail -f /www/wwwlogs/gerbosari.rbwtech.io.log
tail -f /www/wwwlogs/gerbosari.rbwtech.io.error.log
```

## Notes

- DATABASE_URL lives only in `_backend/.env` (mode `0600`, owned by `www`). Never commit the populated file.
- The `_backend/` directory is denied from public HTTP by an explicit `location ^~ /_backend/` block in nginx — confirm via `curl -I https://gerbosari.rbwtech.io/_backend/.env` returns 404.
- aaPanel auto-renews Let's Encrypt; do not hand-edit the `#CERT-APPLY-CHECK` or `#SSL-START` blocks in the site config.
- If you swap to AWS RDS instead of local MySQL, only `DATABASE_URL` changes; nothing else in this runbook moves.
