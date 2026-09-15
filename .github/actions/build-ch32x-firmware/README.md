# Build CH32X Firmware — Composite Action

Builds `firmware/ch32x035-usb-device-compositekm-c` for a given
`keymap.ncl` and board definition, producing `.elf` / `.hex` / `.bin`.

This is the building block behind the reusable workflow
[`build-ch32x-firmware.yaml`](../../workflows/build-ch32x-firmware.yaml).
Use it directly when you need to run the build inside a larger job,
or use the reusable workflow when you just want "give me firmware for this
keymap + board".

## Inputs

| Input | Required | Default | Description |
|---|---|---|---|
| `keymap` | yes | — | Path to `keymap.ncl` (relative to workspace or absolute). |
| `board` | yes | — | Board name (`ch32x-48`, `ch32x-48/rev2025_2`, `ch32x-75`, `ch32x-36-lhs`, `ch32x-36-rhs`, `weact-ch32x-core-board`) **or** path to a custom `board.ncl`. |
| `smart-keymap-dir` | no | `.` | Path to smart-keymap checkout (contains `Cargo.toml`). |
| `firmware-dir` | no | `<smart-keymap-dir>/firmware/ch32x035-usb-device-compositekm-c` | Firmware CMake dir. |
| `build-dir` | no | `build` | CMake build directory (relative to `firmware-dir`). |
| `nickel-version` | no | `1.16.0` | Nickel version. |
| `cbindgen-version` | no | `0.28.0` | cbindgen version. |
| `gcc-version` | no | `14.2.0-3` | xPack `riscv-none-elf-gcc` version. |
| `rust-target` | no | `riscv32imac-unknown-none-elf` | Rust target. |
| `artifact-name` | no | `firmware-ch32x` | Artifact name when uploading. |
| `upload-artifact` | no | `true` | Whether to upload. |

## Outputs

- `firmware-elf`, `firmware-hex`, `firmware-bin`, `build-dir`

## Usage — inside this repo

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: ./.github/actions/build-ch32x-firmware
        with:
          keymap: tests/ncl/keymap-4key-simple/keymap.ncl
          board: ch32x-48
```

## Usage — from an external keyboard repo

Your repo only needs `keymap.ncl` + a board file (`keyboard.ncl`):

```
my-keyboard/
  keymap.ncl
  keyboard.ncl   # board definition, e.g. copy from firmware/.../ncl/boards/ch32x-48.ncl
  .github/workflows/build.yaml
```

```yaml
# .github/workflows/build.yaml in your keyboard repo
name: Build firmware
on: [push, workflow_dispatch]

jobs:
  build:
    uses: rgoulter/smart-keymap/.github/workflows/build-ch32x-firmware.yaml@master
    with:
      keymap: keymap.ncl
      board: keyboard.ncl
```

Or use the composite action directly for more control:

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4  # your keyboard repo
      - uses: actions/checkout@v4  # smart-keymap
        with:
          repository: rgoulter/smart-keymap
          path: smart-keymap
      - uses: ./smart-keymap/.github/actions/build-ch32x-firmware
        with:
          keymap: keymap.ncl
          board: keyboard.ncl
          smart-keymap-dir: smart-keymap
```

The action builds `libsmart_keymap.a` with `SMART_KEYMAP_CUSTOM_KEYMAP`
pointing at your `keymap.ncl`, then configures CMake with `BOARD` pointing
at your board file and produces firmware in `build/` (uploaded as artifact).
Flash the `.hex`/`.bin` with `wchisp`/`wlink` as described in
`firmware/ch32x035-usb-device-compositekm-c/README.MD`.

## Custom boards

Copy one of `firmware/ch32x035-usb-device-compositekm-c/ncl/boards/*.ncl`
as a starting point. The board record is validated against
`keyboard/contracts.ncl`; see that file for the `Board` contract. At minimum
define `board.matrix` (cols, rows, key_count, implementation) and
`board.keymap_index_for_key`.
