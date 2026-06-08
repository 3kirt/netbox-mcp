#!/usr/bin/env bash
# Bring up the test NetBox stack and wait until it's seeded.
#
# Seeding is part of the deployment: the `netbox-token` and `netbox-seed`
# services (see compose.yaml) mint the API token and load scripts/seed_data.py
# automatically once NetBox is healthy. This script just starts everything and
# blocks until the seeder finishes, then prints the connection details.
set -euo pipefail

cd "$(dirname "$0")"

# podman compose (v4+) or the standalone podman-compose, whichever is present.
if podman compose version >/dev/null 2>&1; then
  COMPOSE=(podman compose)
elif command -v podman-compose >/dev/null 2>&1; then
  COMPOSE=(podman-compose)
else
  echo "error: need 'podman compose' or 'podman-compose' on PATH" >&2
  exit 1
fi

URL="http://localhost:8000"
TOKEN="0123456789abcdef0123456789abcdef01234567"

echo ">> starting stack with: ${COMPOSE[*]}"
echo ">> (first boot runs DB migrations + seeding; this can take a few minutes)"
"${COMPOSE[@]}" up -d

echo ">> waiting for the seeder to finish"
seed="netbox-docker-netbox-seed-1"
for _ in $(seq 1 90); do
  state="$(podman inspect "$seed" --format '{{.State.Status}}:{{.State.ExitCode}}' 2>/dev/null || echo pending)"
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
  NETBOX_URL=$URL NETBOX_TOKEN=$TOKEN make test-live
EOF
