Demonstration of using the C interface for `libsmart_keymap` with CH58x
 firmware.

The BLE HID firmware comes from the CH58x EVT examples.

The firmware has been modified to:
- use `libsmart_keymap`,
- and the [WABBLE-60
 keyboard](https://github.com/rgoulter/keyboard-labs/releases/tag/wabble-60-rev2024.2).

## Building

The firmware should be linked using a `libsmart_keymap.a`
 built with the `checkkeys_60key_keymap.rs` keymap
 for the appropriate target.

e.g. with the `just` and the repository's root `justfile`:

```
just build-test-keymap-rv-checkkeys_60key
```

The `include/smart_keymap.h` and
`target/riscv32imac-unknown-none-elf/release/libsmart_keymap.a`
files should be copied into the `libsmartkeymap/` directory.

Matrix key scanning is generated from files under `ncl/`.

To generate the `generated/matrix.c` for the WABBLE-60, run:

```
make
```

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

and flashed using `wchlink`.

### LSP Configuration for clangd

`clangd` can be configured using a file `.clangd` with contents e.g.:

```
CompileFlags:
  Remove:
    - -mcmodel=medany
    - -mabi=*
    - -march=*
    - -mno-save-restore
    - -G
  CompilationDatabase: "build"
```
