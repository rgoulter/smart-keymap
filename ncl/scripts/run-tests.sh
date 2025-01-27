#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -e

SCRIPTS_DIR="$(dirname "$0")"

# Run the nickel checks first.
"${SCRIPTS_DIR}/run-ncl-checks.sh"

# Then with each of the listed `tests/ncl`, check its generated keymap.rs:
#  - matches the expected snapshot,
#  - can be compiled.
ncl_tests=(
    "keymap-1key-simple"
    "keymap-1key-tap_hold"
    "keymap-2key-2layer-simple"
    "keymap-2key-2layer-composite"
    "keymap-48key-basic"
    "keymap-48key-rgoulter"
    "keymap-60key-dvorak-simple"
    "keymap-60key-dvorak-simple-with-tap_hold"
)
for ncl_test in "${ncl_tests[@]}"
do
    "${SCRIPTS_DIR}/test-ncl-diff.sh" "${ncl_test}"
    "${SCRIPTS_DIR}/test-ncl-builds.sh" "${ncl_test}"
done
