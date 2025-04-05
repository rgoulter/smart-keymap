#!/usr/bin/env bash

# $ keymap-ncl-to-json.sh path/to/keymap-directory
#
# Generates a keymap.json file for the keymap.ncl
#  in the given directory.

set -e

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."
NCL_DIR="${REPOSITORY_DIR}/ncl"

KEYMAP_DIR="${1}"

DEST="${KEYMAP_DIR}/keymap.json"

nickel export \
  --format=json \
  --import-path="${KEYMAP_DIR}" \
  --import-path="${NCL_DIR}" \
  keymap-ncl-to-json.ncl \
  keymap.ncl \
  --field="json_keymap" \
  > "${DEST}"
