#!/usr/bin/env sh

# $ test-ncl-builds.sh ncl-test-name
#
# Runs cargo build with the keymap.rs generated from the keymap.json
#  for the given ncl snapshot test.

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."
NCL_TESTS_DIR="${REPOSITORY_DIR}/tests/ncl"

KEYMAP_DIR="${NCL_TESTS_DIR}/${1}"

GENERATED_KEYMAPS="$(find ./tests/ncl -name keymap.rs)"

for GENERATED_KEYMAP in ${GENERATED_KEYMAPS}; do
    KEYMAP_DIR="$(dirname "${GENERATED_KEYMAP}")"
    # Clean: rm the keymap.rs;
    #  and rm the keymap.json if there's a keymap.ncl
    rm --force "${KEYMAP_DIR}/keymap.rs"
    if [ -f "${KEYMAP_DIR}/keymap.ncl" ]; then
        rm --force "${KEYMAP_DIR}/keymap.json"
    fi
done
