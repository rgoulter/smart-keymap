Demonstration of using the C interface for `libsmart_keymap` with
 CH32X035 firmware.

The USB HID firmware comes from the CH32X035 EVT examples.

The firmware has been modified to scan its matrix for
one of these boards, depending on how `generate/matrix.c`
is generated:

- WeAct Studio CH32X035 development board. (PB0, PB1, PB3, PB11).

- CH32X-48.

The firmware has been modified to use `libsmart_keymap`.

## Building

The firmware should be linked using a `libsmart_keymap.a`
 built with a suitable keymap
 for the `riscv32imac-unknown-none-elf` target.

e.g. from the repository's root directory, run:

```
just build-keymap-rv-48key-checkkeys
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

The WeAct core board, for some reason, has code protect enabled, and so requires `wlink` and a
and a WCH link compatible with the CH32X035, such as WCH-LinkE.
(SEE: https://github.com/ch32-rs/wchisp/issues/68#issuecomment-2558167356).

If the CH32X hasn't had code protection enabled, `wchisp` can be used to flash
the MCU by shorting DOWNLOAD and plugging in the USB connector.
