#!/usr/bin/env bash

# $ test-ncl-builds.sh ncl-test-name
#
# Runs cargo build with the keymap.rs generated from the keymap.json
#  for the given ncl snapshot test.

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."
NCL_TESTS_DIR="${REPOSITORY_DIR}/tests/ncl"

KEYMAP_DIR="${NCL_TESTS_DIR}/${1}"

"${SCRIPTS_DIR}/keymap-codegen.sh" "${KEYMAP_DIR}"

SMART_KEYMAP_CUSTOM_KEYMAP=$(realpath "${KEYMAP_DIR}/keymap.rs")

export SMART_KEYMAP_CUSTOM_KEYMAP

cargo build \
    --target riscv32imac-unknown-none-elf \
    --release \
    --no-default-features
