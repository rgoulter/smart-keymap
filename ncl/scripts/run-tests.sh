#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -ex

SCRIPTS_DIR="$(dirname "$0")"

"${SCRIPTS_DIR}/test-ncl-diff.sh" keymap-1key-simple
"${SCRIPTS_DIR}/test-ncl-builds.sh" keymap-1key-simple

