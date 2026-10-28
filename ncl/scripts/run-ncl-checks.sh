#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -e

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."

nickel_eval_checks() {
  nickel \
    eval \
    --import-path="${REPOSITORY_DIR}/ncl" \
    --field="evaluated_checks" \
    checks.ncl \
    "$@"
}

# Modules that export the same field names (e.g. `indices`) cannot be
# merged in one eval; each is checked with checks.ncl on its own.
nickel_eval_checks \
  chording.ncl \
  keymap-ncl-to-json.ncl \
  keymap-codegen.ncl \
  layouts/remap.ncl

nickel_eval_checks sequence.ncl
