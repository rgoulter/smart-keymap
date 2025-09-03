#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -e

SCRIPTS_DIR="$(dirname "$0")"

ncl_tests=(
    "keymap-1key-simple"
)
for ncl_test in "${ncl_tests[@]}"
do
    "${SCRIPTS_DIR}/save-keymap-rs-snapshot.sh" "${ncl_test}"
done
