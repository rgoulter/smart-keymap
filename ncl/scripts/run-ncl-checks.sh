#!/usr/bin/env bash

# Tests the Nickel keymaps under tests/ncl,
#  checking the generated output matches expected snapshots,
#  and that the generated keymap builds.

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."

nickel \
  eval \
  --import-path="${REPOSITORY_DIR}/ncl" \
  --field="evaluated_checks" \
  checks.ncl \
  keymap-ncl-to-json.ncl \
  keymap-codegen.ncl
