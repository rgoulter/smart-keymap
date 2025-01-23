#!/usr/bin/env bash

# $ test-ncl-builds.sh ncl-test-name
#
# Runs cargo build with the keymap.rs generated from the keymap.json
#  for the given ncl snapshot test.

set -e

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

SMART_KEYMAP_CUSTOM_KEYMAP=$(realpath "${KEYMAP_DIR}/keymap.rs")

export SMART_KEYMAP_CUSTOM_KEYMAP

cargo rustc \
    --crate-type "staticlib" \
    --target riscv32imac-unknown-none-elf \
    --release \
    --features "staticlib" \
    --no-default-features
