#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -ex

SCRIPTS_DIR="$(dirname "$0")"

ncl_tests=(
    "keymap-1key-simple"
    "keymap-1key-tap_hold"
    "keymap-2key-2layer-simple"
    "keymap-60key-dvorak-simple"
    "keymap-60key-dvorak-simple-with-tap_hold"
)
for ncl_test in "${ncl_tests[@]}"
do
    "${SCRIPTS_DIR}/save-keymap-rs-snapshot.sh" "${ncl_test}"
done
