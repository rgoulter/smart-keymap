# Smart Keymap task runner — day-to-day DX surface.
#
# Make remains the engine for dependency graphs and CI.
# Day-to-day: `just` / `just choose` / `just test-fast`
# Pre-push:   `just check-quick` (fmt/clippy/doc/nickel) then `just test` when needed
# Full matrix: `just test` (via Make)
#
# Naming: `module::recipe` is Just submodule syntax; multi-word recipes use
# kebab-case (`test-fast`, `fmt-check`). `/` is only for filesystem paths
# (e.g. mod ncl "just/ncl.just"), never recipe names. See AGENTS.md.
#
#   just --list --list-submodules
#   just --choose

mod ncl "just/ncl.just"
mod rust "just/rust.just"
mod ceedling "just/ceedling.just"
mod rp2040-rtic-smart-keyboard

test_keymap := "keymap-4key-simple"
keymap := "tests/ncl/" + test_keymap + "/keymap.ncl"
dest_dir := "firmware/ch32x035-usb-device-compositekm-c/libsmartkeymap/"
target := "riscv32imac-unknown-none-elf"

# ── meta ─────────────────────────────────────────────────────────────

# List recipes (default; does not run the full test matrix)
[group('meta')]
default:
    @just --list --unsorted --list-submodules

# Interactive recipe chooser (fzf / JUST_CHOOSER)
[group('meta')]
choose:
    @just --choose

# ── test aggregates ──────────────────────────────────────────────────

# Full local test matrix via Make (NCL + Rust + Ceedling + lint + cross builds)
[group('test')]
test:
    make test

# Fast daily loop: NCL checks + Rust lib + integration (skips Ceedling, cucumber, snapshots, cross)
[group('test')]
test-fast: ncl::checks rust::lib rust::integration

# Nickel checks + snapshots
[group('test')]
test-ncl: ncl::all

# Nickel evaluated_checks only
[group('test')]
test-ncl-checks: ncl::checks

# Nickel snapshot fixtures
[group('test')]
test-ncl-snapshots: ncl::snapshots

# Rust suites aligned with CI (lib + integration + full-system; not cucumber)
[group('test')]
test-rust: rust::all

# Ceedling C/FFI suites
[group('test')]
test-ceedling: ceedling::all

# ── lint / format / pre-push ─────────────────────────────────────────

# Quick pre-push hygiene: rustfmt, clippy, cargo doc (core+firmware), nickel format
[group('lint')]
check-quick: rust::fmt-check rust::clippy rust::doc ncl::format-check

# rustfmt --check + clippy; also runs ncl-format (writes Nickel sources)
[group('lint')]
lint: rust::fmt-check rust::clippy ncl::format

# Write-format Rust and Nickel sources
[group('lint')]
fmt: rust::fmt ncl::format

# ── build / install (C firmware lib) ─────────────────────────────────

# Generate include/smart_keymap.h via cbindgen
[group('build')]
bindgen:
    cbindgen -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap

# Build libsmart_keymap for the configured keymap / target
[group('build')]
build-keymap:
    env \
      SMART_KEYMAP_CUSTOM_KEYMAP={{ env("SMART_KEYMAP_CUSTOM_KEYMAP", keymap) }} \
        cargo build \
        --release \
        --package "smart_keymap" \
        --target "{{ target }}" \
        --no-default-features

[private]
_install:
    cp include/smart_keymap.h {{ dest_dir }}
    cp target/{{ target }}/release/libsmart_keymap.a {{ dest_dir }}

# cbindgen + build keymap lib + install into ch32x firmware tree
[group('build')]
install: bindgen build-keymap _install

# cargo clean + generated keymaps + headers (make clean)
[group('build')]
clean:
    make clean

# ── release ──────────────────────────────────────────────────────────
#
# cargo-release does the heavy lifting:
#   - version bump (edits [workspace.package])
#   - git commit (with our configured message)
#   - git tag
#
# Push + GitHub release are left explicit for safety.

# Cut a release (version + commit + tag, no publish, no push).
# You can pass an explicit version or the special "release" level (strips -dev).
#
# (just release 0.16.0 | just release  # strips -dev)
[group('release')]
release version="release":
    cargo release -p smart-keymap {{ version }} --no-publish --no-push -x
    @echo ""
    @echo "Local commit + tag created."
    @echo "Next:"
    @echo "  git push origin master && git push origin --tags"
    @echo "  gh release create <the-version> --generate-notes"

# Post-release: bump to the next dev version and commit.
# (cargo-release has a nice "release" level to *strip* -dev, but no symmetric
# one-step "bump + add -dev". So we use cargo set-version here.)
#
# (just bump-dev 0.16.0-dev)
[group('release')]
bump-dev version:
    cargo set-version -p smart-keymap {{ version }}
    git commit -am "cargo: bump version to v{{ version }}"
