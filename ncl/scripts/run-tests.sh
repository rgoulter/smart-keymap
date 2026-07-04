#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap type-checks.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

# Run the nickel checks first.
"${SCRIPTS_DIR}/run-ncl-checks.sh"

mapfile -t ncl_tests < <("${SCRIPTS_DIR}/list-ncl-snapshot-fixtures.sh")

# For each snapshot fixture, check generated keymap.rs:
#  - matches the expected snapshot,
#  - type-checks (cargo check).
for ncl_test in "${ncl_tests[@]}"; do
  "${SCRIPTS_DIR}/test-ncl-diff.sh" "${ncl_test}"
  "${SCRIPTS_DIR}/test-ncl-builds.sh" "${ncl_test}"
done
