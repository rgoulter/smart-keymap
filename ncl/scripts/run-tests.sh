#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

# Run the nickel checks first.
"${SCRIPTS_DIR}/run-ncl-checks.sh"

# For each snapshot fixture, check generated keymap.rs:
#  - matches the expected snapshot,
#  - can be compiled.
while IFS= read -r ncl_test; do
  "${SCRIPTS_DIR}/test-ncl-diff.sh" "${ncl_test}"
  "${SCRIPTS_DIR}/test-ncl-builds.sh" "${ncl_test}"
done < <("${SCRIPTS_DIR}/list-ncl-snapshot-fixtures.sh")
