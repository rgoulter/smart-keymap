Demonstration of using the C interface for `libsmart_keymap` with
 CH32X035 firmware.

The USB HID firmware comes from the CH32X035 EVT examples.

The firmware has been modified to use pads broken out on the
 WeAct Studio CH32X035 development board. (PB0, PB1, PB3, PB11).

The firmware has been modified to use `libsmart_keymap`.

## Building

The firmware should be linked using a `libsmart_keymap.a`
 built with a keymap with at least 4 keys,
 for the appropriate target.

e.g. from the repository's root directory, run:

```
env SMART_KEYMAP_CUSTOM_KEYMAP="$(pwd)/tests/keymaps/simple_keymap.rs" \
  cargo build \
    --target riscv32imac-unknown-none-elf \
    --release \
    --no-default-features
```

The `include/smart_keymap.h` and
`target/riscv32imac-unknown-none-elf/release/libsmart_keymap.a`
files should be copied into the `libsmartkeymap/` directory.

An appropriate toolchain should be on `PATH`. e.g. using the package from the
Nix flake from <https://github.com/rgoulter/ch32>:

```
nix shell github:rgoulter/ch32#mrs-riscv-embedded-gcc12
```

Then the firmware can be built with:

```
cmake --toolchain=../../toolchains/riscv-none-elf.cmake ..
make
```

and flashed using `wlink` with a WCH link compatible with the CH32X035, such as WCH-LinkE.
