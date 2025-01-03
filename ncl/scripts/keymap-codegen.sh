#!/usr/bin/env bash

# $ keymap-codegen.sh path/to/keymap-directory
#
# Generates a formatted Rust keymap.rs file for the keymap.json
#  in the given directory.

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."
NCL_DIR="${REPOSITORY_DIR}/ncl"

KEYMAP_DIR="${1}"

DEST="${KEYMAP_DIR}/keymap.rs"

nickel export \
  --format=raw \
  --import-path="${KEYMAP_DIR}" \
  "${NCL_DIR}/keymap-codegen.ncl" \
  > "${DEST}"

rustfmt "${DEST}"
