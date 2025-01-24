test_keymap := "keymap-4key-simple"

dest_dir := "firmware/ch32x035-usb-device-compositekm-c/libsmartkeymap/"

target := "riscv32imac-unknown-none-elf"

default: test

bindgen:
	cbindgen -c cbindgen.toml -o include/smart_keymap.h

clean:
    make clean

test:
    make test

build-keymap:
    env \
      SMART_KEYMAP_CUSTOM_KEYMAP={{env("SMART_KEYMAP_CUSTOM_KEYMAP", "tests/ncl/" + test_keymap + "/keymap.ncl")}} \
        cargo rustc \
        --crate-type "staticlib" \
        --release \
        --target "{{target}}" \
        --no-default-features \
        --features "staticlib"

_install:
    cp include/smart_keymap.h {{dest_dir}}
    cp target/{{target}}/release/libsmart_keymap.a {{dest_dir}}

install: bindgen build-keymap _install
