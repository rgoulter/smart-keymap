#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -e

SCRIPTS_DIR="$(dirname "$0")"

ncl_tests=(
    "keymap-1key-simple"
    "keymap-1key-tap_dance"
    "keymap-1key-tap_hold"
    "keymap-1key-custom"
    "keymap-1key-2layer-th-lmod"
    "keymap-1key-callback-custom"
    "keymap-2key-2layer-simple"
    "keymap-2key-2layer-composite"
    "keymap-2key-chorded"
    "keymap-48key-basic"
    "keymap-48key-rgoulter"
    "keymap-60key-dvorak-simple"
    "keymap-60key-dvorak-simple-with-tap_hold"
)
for ncl_test in "${ncl_tests[@]}"
do
    "${SCRIPTS_DIR}/save-keymap-rs-snapshot.sh" "${ncl_test}"
done
