#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap type-checks.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"

"${SCRIPTS_DIR}/run-ncl-checks.sh"
"${SCRIPTS_DIR}/run-snapshots.sh"
