default: test

bindgen:
	cbindgen -c cbindgen.toml -o include/smart_keymap.h

clean:
    make clean

test:
    make test

build-test-keymap-rv-checkkeys_60key: (build-keymap-rv `pwd`/"tests/keymaps/checkkeys_60key_keymap.rs")

build-keymap-rv $SMART_KEYMAP_CUSTOM_KEYMAP: (build-keymap-target SMART_KEYMAP_CUSTOM_KEYMAP "riscv32imac-unknown-none-elf")

build-keymap-target $SMART_KEYMAP_CUSTOM_KEYMAP target:
    cargo build \
        --target riscv32imac-unknown-none-elf \
        --release \
        --no-default-features \
        --features "usbd-human-interface-device"

