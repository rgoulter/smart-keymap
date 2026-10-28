#!/usr/bin/env bash

# Lists tests/ncl fixture directories that have expected.rs snapshots.
# One fixture name per line, sorted for stable ordering.

set -euo pipefail

SCRIPTS_DIR="$(dirname "$0")"
NCL_TESTS_DIR="${SCRIPTS_DIR}/../../tests/ncl"

for fixture_dir in "${NCL_TESTS_DIR}"/keymap-*; do
  [[ -d "${fixture_dir}" ]] || continue
  [[ -f "${fixture_dir}/expected.rs" ]] || continue
  basename "${fixture_dir}"
done | sort
