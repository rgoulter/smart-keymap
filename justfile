default: test

bindgen:
	cbindgen -c cbindgen.toml -o include/smart_keymap.h

clean:
    make clean

test:
    make test

build-test-keymap-rv-ncl_60key_dvorak: && (build-keymap-rv `pwd`/"tests/ncl/keymap-60key-dvorak-simple/keymap.rs")
    make tests/ncl/keymap-60key-dvorak-simple/keymap.rs

build-test-keymap-rv-checkkeys_60key: (build-keymap-rv `pwd`/"tests/keymaps/checkkeys_60key_keymap.rs")

build-keymap-rv $SMART_KEYMAP_CUSTOM_KEYMAP: (build-keymap-target SMART_KEYMAP_CUSTOM_KEYMAP "riscv32imac-unknown-none-elf")

build-keymap-target $SMART_KEYMAP_CUSTOM_KEYMAP target:
    cargo rustc \
        --crate-type "staticlib" \
        --target riscv32imac-unknown-none-elf \
        --release \
        --no-default-features \
        --features "staticlib" \
        --features "usbd-human-interface-device"
