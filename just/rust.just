# Rust unit, integration, cucumber, and lint recipes.
# Names align with .github/workflows/rust.yaml job splits.
# Recipes run from the repository root.

set working-directory := ".."

# Unit tests: smart-keymap-core + smart-keymap libs
[group('rust')]
lib:
    cargo test -p smart-keymap-core --lib
    cargo test -p smart-keymap --lib

# smart-keymap-core lib tests, optional filter
# Example: just rust::core keymap::
[group('rust')]
core filter="":
    cargo test -p smart-keymap-core --lib {{ filter }}

# smart-keymap facade lib tests, optional filter
[group('rust')]
facade filter="":
    cargo test -p smart-keymap --lib {{ filter }}

# Integration suite (tests/rust via --test rust-integration)
# (just rust::integration | just rust::integration tap_hold | just rust::integration chorded::overlapping)
[group('rust')]
integration module="":
    cargo test --test rust-integration {{ module }}

# fzf-pick a top-level integration module under tests/rust/
[group('rust')]
integration-pick:
    #!/usr/bin/env bash
    set -euo pipefail
    module="$(
      find tests/rust -mindepth 1 -maxdepth 1 \( -name '*.rs' -o -type d \) \
        ! -name 'keymap.rs' ! -name 'hid_keycodes.rs' \
        | sed 's|tests/rust/||; s|\.rs$||' \
        | sort -u \
        | fzf --prompt='rust integration> '
    )"
    test -n "${module}"
    just rust::integration "${module}"

# Full-system keymap tests (smart-keymap-full-system-std)
[group('rust')]
full-system filter="":
    cargo test -p smart-keymap-full-system-std --test keymap_full_system {{ filter }}

# Cucumber keymap features (slow)
# Example: just rust::cucumber sequence
[group('rust')]
cucumber filter="":
    cargo test -p smart-keymap-full-system-std --test cucumber-keymap {{ filter }}

# Lib + integration + full-system (CI-ish; excludes cucumber)
[group('rust')]
all: lib integration full-system

# cargo test workspace default (broader than CI splits)
[group('rust')]
workspace:
    make test-rust

# Match Makefile / CI clippy excludes
[group('rust')]
clippy:
    make test-clippy

# cargo fmt --check
[group('rust')]
fmt-check:
    make test-fmt

# cargo fmt (write)
[group('rust')]
fmt:
    cargo fmt --all

# cargo doc with deny warnings (core packages)
[group('rust')]
doc:
    make test-doc
