#!/usr/bin/env bash

# $ ceedling-fixture.sh path/to/keymap-directory
#
# Emits a C header of #defines from the keymap's ceedling_fixture field.

set -e

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../../.."
NCL_DIR="${REPOSITORY_DIR}/ncl"
CEEDLING_NCL_DIR="${SCRIPTS_DIR}/../ncl"

KEYMAP_DIR="${1}"

nickel export \
  --format=raw \
  --import-path="${KEYMAP_DIR}" \
  --import-path="${NCL_DIR}" \
  "${CEEDLING_NCL_DIR}/ceedling-fixture.ncl" \
  --field="ceedling_fixture_h"
