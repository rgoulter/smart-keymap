#!/usr/bin/env bash

# Snapshot fixtures: export keymap.json/keymap.rs and diff against expected.rs,
# then cargo-check each generated keymap.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

while IFS= read -r ncl_test; do
  "${SCRIPTS_DIR}/test-ncl-diff.sh" "${ncl_test}"
  "${SCRIPTS_DIR}/test-ncl-builds.sh" "${ncl_test}"
done < <("${SCRIPTS_DIR}/list-ncl-snapshot-fixtures.sh")
