#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -ex

SCRIPTS_DIR="$(dirname "$0")"

ncl_tests=(
    "keymap-1key-simple"
    "keymap-60key-dvorak-simple"
)
for ncl_test in "${ncl_tests[@]}"
do
    "${SCRIPTS_DIR}/test-ncl-diff.sh" "${ncl_test}"
    "${SCRIPTS_DIR}/test-ncl-builds.sh" "${ncl_test}"
done
