#!/usr/bin/env bash

# $ test-ncl-diff.sh ncl-test-name
#
# Runs diff for the keymap.rs generated from the keymap.json
#  against the expected.rs in the directory for
#  the given ncl snapshot test.

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."
NCL_TESTS_DIR="${REPOSITORY_DIR}/tests/ncl"

KEYMAP_DIR="${NCL_TESTS_DIR}/${1}"

# Clean: rm the keymap.rs;
#  and rm the keymap.json if there's a keymap.ncl
rm --force "${KEYMAP_DIR}/keymap.rs"
if [[ -f "${KEYMAP_DIR}/keymap.ncl" ]]; then
    rm --force "${KEYMAP_DIR}/keymap.json"
fi

pushd "${REPOSITORY_DIR}"
    make "${KEYMAP_DIR}/keymap.json"
    make "${KEYMAP_DIR}/keymap.rs"
popd

cp "${KEYMAP_DIR}/keymap.rs" "${KEYMAP_DIR}/expected.rs"
