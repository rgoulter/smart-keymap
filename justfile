test_keymap := "keymap-4key-simple"

keymap := "tests/ncl/" + test_keymap + "/keymap.ncl"

dest_dir := "firmware/ch32x035-usb-device-compositekm-c/libsmartkeymap/"

target := "riscv32imac-unknown-none-elf"

mod rp2040-rtic-smart-keyboard

default: test

bindgen:
	cbindgen -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap

clean:
    make clean

test:
    make test

build-keymap:
    env \
      SMART_KEYMAP_CUSTOM_KEYMAP={{env("SMART_KEYMAP_CUSTOM_KEYMAP", keymap)}} \
        cargo build \
        --release \
        --package "smart_keymap" \
        --target "{{target}}" \
        --no-default-features

_install:
    cp include/smart_keymap.h {{dest_dir}}
    cp target/{{target}}/release/libsmart_keymap.a {{dest_dir}}

install: bindgen build-keymap _install

# === Release helpers ===
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
# Examples:
#   just release 0.16.0
#   just release                  # 0.15.0-dev → 0.15.0 (recommended)
release version="release":
	cargo release -p smart-keymap {{version}} --no-publish --no-push -x
	@echo ""
	@echo "Local commit + tag created."
	@echo "Next:"
	@echo "  git push origin master && git push origin --tags"
	@echo "  gh release create <the-version> --generate-notes"

# Post-release: bump to the next dev version and commit.
# (cargo-release has a nice "release" level to *strip* -dev, but no symmetric
# one-step "bump + add -dev". So we use cargo set-version here.)
#
# Examples:
#   just bump-dev 0.16.0-dev
#   just bump-dev 0.15.1-dev   # if you prefer patch-level dev
bump-dev version:
	cargo set-version -p smart-keymap {{version}}
	git commit -am "cargo: bump version to v{{version}}"
