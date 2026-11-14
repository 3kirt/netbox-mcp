#!/usr/bin/env bash
# Bring up the test NetBox stack and wait until it's seeded.
#
# Seeding is part of the deployment: the `netbox-token` and `netbox-seed`
# services (see compose.yaml) mint the API token and load scripts/seed_data.py
# automatically once NetBox is healthy. This script just starts everything and
# blocks until the seeder finishes, then prints the connection details.
set -euo pipefail

cd "$(dirname "$0")"

# Container engine: podman (the local default on macOS) or docker (GitHub
# Actions runners). ENGINE=docker|podman forces one. The compose project and
# container names are the same under both, so the rest of the script is shared.
pick_engine() {
  if podman compose version >/dev/null 2>&1; then
    ENGINE=podman; COMPOSE=(podman compose)
  elif command -v podman-compose >/dev/null 2>&1; then
    ENGINE=podman; COMPOSE=(podman-compose)
  elif docker compose version >/dev/null 2>&1; then
    ENGINE=docker; COMPOSE=(docker compose)
  else
    echo "error: need 'podman compose', 'podman-compose', or 'docker compose' on PATH" >&2
    exit 1
  fi
}
case "${ENGINE:-}" in
  docker) COMPOSE=(docker compose) ;;
  podman) COMPOSE=(podman compose) ;;
  "") pick_engine ;;
  *) echo "error: ENGINE must be docker or podman, got '$ENGINE'" >&2; exit 1 ;;
esac

URL="http://localhost:8000"
TOKEN="0123456789abcdef0123456789abcdef01234567"

echo ">> starting stack with: ${COMPOSE[*]}"
echo ">> (first boot runs DB migrations + seeding; this can take a few minutes)"
"${COMPOSE[@]}" up -d

echo ">> waiting for the seeder to finish"
seed="netbox-docker-netbox-seed-1"
for _ in $(seq 1 90); do
  state="$("$ENGINE" inspect "$seed" --format '{{.State.Status}}:{{.State.ExitCode}}' 2>/dev/null || echo pending)"
  case "$state" in
    exited:0) echo ">> seeding complete"; break ;;
    exited:*) echo "error: seeder failed ($state); see '${COMPOSE[*]} logs netbox-seed'" >&2; exit 1 ;;
    *)        sleep 5 ;;
  esac
done

if [ "${state:-}" != "exited:0" ]; then
  echo "error: seeder did not finish in time; check '${COMPOSE[*]} logs netbox-seed'" >&2
  exit 1
fi

cat <<EOF

Done. NetBox is up and seeded.
  UI:    $URL  (login: admin / admin)
  URL:   $URL
  TOKEN: $TOKEN

Run the live suite from the repo root:
  NETBOX_URL=$URL NETBOX_TOKEN=$TOKEN make live-test
EOF
