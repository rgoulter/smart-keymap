# Smart Keymap

Build keyboard firmware by declaring keymaps in [Nickel](https://nickel-lang.org/), powered by [Rust](rust-lang.org).

A 'smart keyboard' is a keyboard with additional bells and whistles, such as
 alternate key functionality (layers, tap-hold keys, chords, etc.), or RGB
 effects, etc. -- e.g. [QMK](https://docs.qmk.fm/), [ZMK](https://zmk.dev/docs/)
 are popular smart keyboard firmware frameworks.
 
This project provides a library which handles the keymap behaviour part of this.

- Nickel allows for concise expression of keymaps, as [semicolon's
  fak](https://github.com/semickolon/fak) showed. (e.g. [simple example
  keymap](https://github.com/rgoulter/smart-keymap/blob/master/tests/ncl/keymap-48key-basic/keymap.ncl),
  [a more sophisticated example
  keymap](https://github.com/rgoulter/smart-keymap/blob/master/tests/ncl/keymap-36key-rgoulter/keymap.ncl)).
  Nickel is a configuration language along the lines of "JSON + functions"; its
  [emphasis on correctness](https://nickel-lang.org/user-manual/correctness/)
  and [modular
  configurations](https://nickel-lang.org/user-manual/modular-configurations/)
  make it a powerful configuration language.

- This project supports the cheap and powerful
  [CH32X035](https://www.wch-ic.com/products/CH32X035.html), a 32-bit RISC-V
  MCU. -- In the same way that [semicolon's
  fak](https://github.com/semickolon/fak) enabled low-budget keyboard designs
  with the  [CH552](https://www.wch-ic.com/products/CH552.html), this project
  enables keyboard designs using CH32X035.

- The smart keymap library can be built as a static native library, which makes
  it easy to integrate into USB HID keyboard examples written in C.

- As Rust crate, the project also supports using Rust to write keyboard
  firmware, similar to what [keyberon](https://github.com/TeXitoi/keyberon)
  supports.

- This project also draws inspiration from ideas explored in [semickolon's
  kirei](https://github.com/semickolon/kirei), which emphasised "keys as the
  main abstraction of keymap behaviour".
  
## Usage

The main idea is write a keymap in Nickel (`keymap.ncl`),
 and build keyboard firmware which uses `smart-keymap` with this custom keymap.

See [the RP2040 firmware example below](#building-with-custom-keymap-and-board)

## Documentation and Examples

Documentation for features which have been implemented can be found at:
<https://rgoulter.com/smart-keymap/features.html>

Some sample ncl keymaps can be found under [tests/ncl/](https://github.com/rgoulter/smart-keymap/tree/master/tests/ncl). In particular:

- [48-key basic keymap](tests/ncl/keymap-48key-basic/keymap.ncl),
- [36-key rgoulter keymap](tests/ncl/keymap-36key-rgoulter/keymap.ncl).
  - used as a [48-key keymap](tests/ncl/keymap-48key-rgoulter/keymap.ncl).

Published documentation for the Rust crate can be found at <https://rgoulter.com/smart-keymap/doc/smart_keymap/index.html>.

## Keyboard Firmware

Some keyboard firmware which uses `smart-keymap`:

### RP2040 RTIC Keyboard firmware

The firmware under `rp2040-rtic-smart-keyboard` has been adapted from the [rgoulter/keyboard-labs firmware/keyberon](https://github.com/rgoulter/keyboard-labs/tree/master/firmware/keyberon) code.

In particular, the firmware under `rp2040-rtic-smart-keyboard` has [an example](rp2040-rtic-smart-keyboard/examples/pico42.rs) for the
[Pico42](https://github.com/rgoulter/keyboard-labs/releases/tag/pico42-rev2023.2).

#### Building

##### Dependencies

[DevEnv](https://devenv.sh/) is used to provide the toolchain dependencies.

Use `devenv shell` to enter a shell which has all the tooling installed.

A [DevContainer](https://containers.dev/) is defined for the project, which can be used to easily get started using [e.g. VSCode, GitHub Codespaces, etc.](https://containers.dev/supporting).

##### Smart Keymap Dependencies

Using a `keymap.ncl` (or the `keymap.rs` generated from its definition),
the keyboard firmware can be built with a command such as:

```
env SMART_KEYMAP_CUSTOM_KEYMAP="$(pwd)/tests/ncl/keymap-42key-dvorak-simple-with-tap_hold/keymap.ncl" \
  cargo build \
    --release \
    --target=thumbv6m-none-eabi \
    --package=rp2040-rtic-smart-keyboard
```

The firmware can be deployed to an RP2040 board (in bootloader mode)
 by using `cargo run` instead of `cargo build`.
 
The `pico42` example can be built / run by adding `--example pico42`.

A custom `board.ncl` file can be built by setting the `SMART_KEYBOARD_CUSTOM_BOARD`
variable to its path, and building the `rp2040-rtic-smart-keyboard` package's binary. 

##### Building with Custom Keymap and Board

e.g. with [42key-dvorak/keymap.ncl](tests/ncl/keymap-42key-dvorak-simple-with-tap_hold/keymap.ncl),

```
let K = import "keys.ncl" in

# Define tap_hold keys
# by merging a `K.hold` modifier
# with a key.
let A_A = K.A & K.hold K.LeftAlt in
let G_O = K.O & K.hold K.LeftGUI in
let C_E = K.E & K.hold K.LeftCtrl in
let S_U = K.U & K.hold K.LeftShift in
let S_H = K.H & K.hold K.RightShift in
let C_T = K.T & K.hold K.RightCtrl in
let G_N = K.N & K.hold K.RightGUI in
let A_S = K.S & K.hold K.RightAlt in

{
  keys = [
    K.QUOT, K.COMM, K.DOT,  K.P,   K.Y,                  K.F,   K.G,   K.C,    K.R,    K.L,
    A_A,    G_O,    C_E,    S_U,   K.I,                  K.D,   S_H,   C_T,    G_N,    A_S,
    K.SCLN, K.Q,    K.J,    K.K,   K.X,                  K.B,   K.M,   K.W,    K.V,    K.Z,
    K.LCTL, K.LGUI, K.LALT, K.TAB, K.ESC, K.SPC, K.BSPC, K.RET, K.DEL, K.RALT, K.RGUI, K.RCTL,
  ],
}
```

and [board-pico42.ncl](rp2040-rtic-smart-keyboard/examples/board-pico42.ncl)
(for the [Pico42](https://github.com/rgoulter/keyboard-labs/releases/tag/pico42-rev2023.2)).

```
{
  gpio_pins,

  usb
    = {
      vid = 0xCAFE,
      pid = 0x0005,
      manufacturer = "rgoulter keyboard-labs",
      product = "Pico42"
    },

  matrix
    =
      let p = gpio_pins in
      {
        cols = [p.GP0, p.GP1, p.GP2, p.GP3, p.GP4, p.GP5, p.GP6, p.GP7, p.GP8, p.GP9, p.GP10, p.GP11],
        rows = [p.GP14, p.GP15, p.GP16, p.GP17],
        key_count = 42,
      },

  keymap_index_for_key = fun { column_index, row_index } =>
    let NO = null in
    let keymap_indices = [
      [ 0,  1,  2,  3,  4, NO, NO,  5,  6,  7,  8,  9],
      [10, 11, 12, 13, 14, NO, NO, 15, 16, 17, 18, 19],
      [20, 21, 22, 23, 24, NO, NO, 25, 26, 27, 28, 29],
      [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41],
    ]
    in
    let row = std.array.at row_index keymap_indices in
    std.array.at column_index row |> match {
        idx if idx != null => 'Key idx,
        _ => 'NoKey,
    },
}
```

the keyboard firmware can be built & flashed to the RP2040 bootloader with:

```
env \
  SMART_KEYMAP_CUSTOM_KEYMAP="$(pwd)/tests/ncl/keymap-42key-dvorak-simple-with-tap_hold/keymap.ncl" \
  SMART_KEYBOARD_CUSTOM_BOARD="$(pwd)/rp2040-rtic-smart-keyboard/examples/board-pico42.ncl" \
    cargo run \
      --release \
      --target=thumbv6m-none-eabi \
      --package=rp2040-rtic-smart-keyboard
```

### CH32X035 USB HID Keyboard

The example USB HID keyboard software from the EVT has been adapted
to work with smart_keymap lib.

See [firmware/ch32x035-usb-device-compositekm-c](firmware/ch32x035-usb-device-compositekm-c/README.MD) for more details.
