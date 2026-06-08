# Test NetBox stack (Podman on macOS)

A self-contained NetBox stack for exercising netbox-mcp against a real instance —
used by the live test suite (`make test-live`, see [`docs/testing.md`](../../docs/testing.md))
and for manual checks via [`docs/testing-protocol.md`](../../docs/testing-protocol.md).

It's a trimmed copy of [netbox-community/netbox-docker](https://github.com/netbox-community/netbox-docker),
pre-wired with a **fixed API token** and **automatic seeding** so testing needs no
manual setup. Credentials here are throwaway and for local testing only.

> Files here are derived from netbox-docker, which is licensed under
> **Apache-2.0** (compatible with this project's GPL-3.0). See [`LICENSE`](LICENSE)
> for the license text and [`NOTICE`](NOTICE) for attribution and the list of
> files copied or modified.

| | |
|---|---|
| URL | `http://localhost:8000` |
| API token | `0123456789abcdef0123456789abcdef01234567` |
| Admin login | `admin` / `admin` |

Seeding is part of the deployment. Two one-shot services run after NetBox is
healthy:

- **`netbox-token`** mints a legacy **v1** API token (`Token <key>` scheme) for
  the admin user. NetBox 4.6's built-in superuser bootstrap only creates v2
  (`Bearer`) tokens, but netbox-mcp authenticates with the v1 scheme — so the
  token is created directly via `manage.py`.
- **`netbox-seed`** then loads `scripts/seed_data.py`. It's idempotent, so it's
  safe to re-run on every `up`.

## Prerequisites

- Podman with a running machine (`podman machine init && podman machine start`)
- `podman compose` (Podman 4+) or `podman-compose`

Seeding runs inside a container, so no host Python/pynetbox is required.

## Quick start

```sh
cd test/netbox-docker
./up.sh
```

`up.sh` starts the stack and blocks until the seeder finishes (first boot runs
database migrations + seeding — give it a few minutes), then prints the
connection details.

### Manual equivalent

```sh
podman compose up -d        # boots NetBox, mints the token, and seeds
podman compose logs -f netbox-seed   # watch seeding progress
```

## Running the live tests

From the repo root, once the stack is seeded:

```sh
NETBOX_URL=http://localhost:8000 \
NETBOX_TOKEN=0123456789abcdef0123456789abcdef01234567 \
  make test-live
```

## Managing the stack

```sh
podman compose logs -f netbox     # follow logs (watch first-boot migrations)
podman compose ps                 # service status / health
podman compose down               # stop, keep data volumes
podman compose down -v            # stop and wipe all data (fresh start next time)
```

## Notes for Podman on macOS

- Image names are fully qualified (`docker.io/...`) because Podman doesn't
  assume a default registry.
- The stack is rootless-friendly: NetBox publishes on port 8000 (>1024, no root
  needed) and the config mount uses the `:z` SELinux relabel flag, which Podman
  honors inside its Linux VM and Docker ignores.
- If `podman compose` isn't available, install the standalone tool:
  `pip install podman-compose` (the `up.sh` helper detects either).
- Pin a different NetBox image with `VERSION=v4.6-5.0.1 podman compose up -d`.
