#!/usr/bin/env bash

# Refreshes expected.rs snapshots for tests/ncl fixtures.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

mapfile -t ncl_tests < <("${SCRIPTS_DIR}/list-ncl-snapshot-fixtures.sh")

for ncl_test in "${ncl_tests[@]}"; do
  "${SCRIPTS_DIR}/save-keymap-rs-snapshot.sh" "${ncl_test}"
done
