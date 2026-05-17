# AWS EC2 Setup

This project deploys to an aaPanel-managed host at `gerbosari.rbwtech.io`.
See [aapanel-setup.md](aapanel-setup.md) for the current runbook.

The previous bare-Ubuntu / nginx + certbot recipe lived here; it diverged
from the live infrastructure (different paths, different service user) and
was removed to avoid drift. If you need a non-aaPanel deployment, derive it
from `aapanel-setup.md` and the systemd / nginx files in this folder — the
only host-specific bits are the paths under `/www/wwwroot/...` and the
`www` user.
