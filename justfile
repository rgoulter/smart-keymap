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
