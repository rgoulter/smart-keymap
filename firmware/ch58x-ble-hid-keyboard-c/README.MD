Demonstration of using the C interface for `libsmart_keymap` with CH58x
 firmware.

The BLE HID firmware comes from the CH58x EVT examples.

The firmware has been modified to:
- use `libsmart_keymap`,
- and the [WABBLE-60
 keyboard](https://github.com/rgoulter/keyboard-labs/releases/tag/wabble-60-rev2024.2).

## Dependencies

With [devenv](https://devenv.sh/) and [direnv](https://direnv.net/) installed,
 in this directory run:

```
direnv allow
```

And the required tools will be downloaded, & available in the shell.

## Building

### libsmart_keymap

The firmware should be linked using a `libsmart_keymap.a`
 built with the `checkkeys_60key_keymap.rs` keymap
 for the appropriate target.

e.g. with the `just` and the repository's root `justfile`:

```
just \
  dest_dir=firmware/ch58x-ble-hid-keyboard-c/libsmartkeymap/ \
  keymap=tests/ncl/keymap-60key-rgoulter/keymap.ncl \
  install
```

The `include/smart_keymap.h` and
`target/riscv32imac-unknown-none-elf/release/libsmart_keymap.a`
files are copied into the `libsmartkeymap/` directory.

### Board-specific Codegen

Matrix key scanning is generated from files under `ncl/`.

To generate this for the CH58X BLE HID firmware, in this directory run:

```
just board=ncl/boards/wabble-60.ncl build
```

To flash, similarly:

```
just board=ncl/boards/wabble-60.ncl flash
```

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
