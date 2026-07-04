#!/usr/bin/env bash

# Refreshes expected.rs snapshots for tests/ncl fixtures.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

while IFS= read -r ncl_test; do
  "${SCRIPTS_DIR}/save-keymap-rs-snapshot.sh" "${ncl_test}"
done < <("${SCRIPTS_DIR}/list-ncl-snapshot-fixtures.sh")
