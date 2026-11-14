#!/usr/bin/env bash
# Print the body of a CHANGELOG.md section: everything between
# `## [<version>] - YYYY-MM-DD` (or `## [Unreleased]`) and the next `## [`.
#
#   scripts/changelog-section.sh 0.14.0 [CHANGELOG.md]
#
# Exits non-zero if the section is missing or has no content, so the release
# workflow can refuse to publish a version without notes. The same script feeds
# the GitHub release body.
set -euo pipefail

version="${1:?usage: changelog-section.sh <version> [changelog]}"
file="${2:-$(dirname "$0")/../CHANGELOG.md}"

body="$(awk -v ver="$version" '
  BEGIN { want = "## [" ver "]" }
  index($0, "## [") == 1 {
    if (found) exit
    if (substr($0, 1, length(want)) == want) { found = 1; next }
  }
  found { print }
' "$file")"

# Trim leading/trailing blank lines.
body="$(printf '%s\n' "$body" | sed -e '/./,$!d' | sed -e ':a' -e '/^\n*$/{$d;N;ba' -e '}')"

if [ -z "$(printf '%s' "$body" | tr -d '[:space:]')" ]; then
  echo "changelog-section: no non-empty section for [$version] in $file" >&2
  exit 1
fi
printf '%s\n' "$body"
