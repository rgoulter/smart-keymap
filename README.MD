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

The main idea is write a keymap in Nickel (`keymap.ncl`), and build keyboard
 firmware which uses `smart-keymap` with this custom keymap, either using
 smart-keymap as a Rust crate, or using `libsmart_keymap` as a native library.

See the [keyboard firmware](#keyboard-firmware) section below for keyboard
 firmware which uses smart-keymap.

To make use of smart-keymap in a "user config" repository, see
[rgoulter/smart-keyboard-config](https://github.com/rgoulter/smart-keyboard-config).

## Documentation and Examples

Documentation for features which have been implemented can be found at:
<https://rgoulter.com/smart-keymap/features.html>

Some sample ncl keymaps can be found under [tests/ncl/](https://github.com/rgoulter/smart-keymap/tree/master/tests/ncl). In particular:

- [48-key basic keymap](tests/ncl/keymap-48key-basic/keymap.ncl)
- [36-key rgoulter keymap](tests/ncl/keymap-36key-rgoulter/keymap.ncl)
  - used as a [48-key keymap](tests/ncl/keymap-48key-rgoulter/keymap.ncl)
- [34-key seniply keymap](tests/ncl/keymap-34key-seniply/keymap.ncl)
- [36-key miryoku keymap](tests/ncl/keymap-36key-miryoku/keymap.ncl)

Published documentation for the Rust crates can be found at <https://rgoulter.com/smart-keymap/doc/smart_keymap/index.html>.

### Keyboards using Smart Keymap

[Pico42](https://github.com/rgoulter/keyboard-labs#pico42-simple-hand-solderable-pcb-in-bm40jj40-form-factor),
[rp2040-rtic-smart-keyboard firmware](https://github.com/rgoulter/smart-keymap/tree/master/rp2040-rtic-smart-keyboard).

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/refs/heads/master/docs/images/keyboards/pico42/pico42-mx_lowprofile_case.JPG" width=600 />

[CH32X-36](https://github.com/rgoulter/keyboard-labs#ch32x-36-low-budget-36-key-split-keyboard-with-smt-components),
[ch32x035-usb-device-compositekm-c firmware](https://github.com/rgoulter/smart-keymap/tree/master/firmware/ch32x035-usb-device-compositekm-c)

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/master/docs/images/keyboards/ch32x-36/ch32x-36.JPG" width=600 />

[CH32X-48](https://github.com/rgoulter/keyboard-labs#ch32x-48-low-budget-pcba-in-bm40jj40-form-factor),
[ch32x035-usb-device-compositekm-c firmware](https://github.com/rgoulter/smart-keymap/tree/master/firmware/ch32x035-usb-device-compositekm-c)

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/master/docs/images/keyboards/ch32x-48/ch32x-48.JPG" width=600 />

[CH32X-75-LPR](https://github.com/rgoulter/keyboard-labs#ch32x-75-lpr-low-budget-pcba-with-low-profile-redragon-switches),
[ch32x035-usb-device-compositekm-c firmware](https://github.com/rgoulter/smart-keymap/tree/master/firmware/ch32x035-usb-device-compositekm-c)

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/master/docs/images/keyboards/ch32x-75-lpr/ch32x-75.JPG" width=600 />

[WABBLE-60](https://github.com/rgoulter/keyboard-labs#wabble-60-5x12-ortholinear-ble-risc-v-powered-adaptation-of-peejs-lumberjack-pcb),
[ch58x-ble-hid-keyboard-c firmware](https://github.com/rgoulter/smart-keymap/tree/master/firmware/ch58x-ble-hid-keyboard-c)

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/master/docs/images/keyboards/wabble-60/wabble-60.JPG" width=600 />

[MiniF4-36](https://github.com/rgoulter/keyboard-labs#x-1-36-key-split-reversible-pcb-with-weact-studio-mini-f4-dev-board),
[stm32f4-rtic-smart-keyboard firmware](https://github.com/rgoulter/smart-keymap/tree/master/stm32f4-rtic-smart-keyboard),
[stm32-embassy-smart-keyboard firmware](https://github.com/rgoulter/smart-keymap/tree/master/stm32-embassy-smart-keyboard),

<img src="https://raw.githubusercontent.com/rgoulter/keyboard-labs/master/docs/images/photo-keyboard-x-1-mx-tented.JPG" width=600 />

See the [Keyboards wiki page](https://github.com/rgoulter/smart-keymap/wiki/Keyboards).

## Comparison to Other Keyboard Firmware

Smart Keymap provides a library of functions. In this sense, it's most similar
 to [TeXitoi's keyberon](https://github.com/TeXitoi/keyberon), where the user
 has to implement their own keyboard firmware. -- This is in contrast to popular
 customisable keyboard firmwares such as [QMK](https://qmk.fm/) or
 [ZMK](https://zmk.dev/), which are frameworks.

Configuring keymaps is done using [Nickel](https://nickel-lang.org/).
 Keyberon's keymaps are written directly in Rust,
 QMK's keymaps are written either in C, or with its data-driven configuration using JSON.
 ZMK's keymaps are written with [devicetree](https://www.devicetree.org/).

Smart Keymap draws a lot of inspiration from [semickolon's fak](https://github.com/semickolon/fak)
 and [kirei](https://github.com/semickolon/kirei) projects, which also use Nickel.
 Similar to these projects, Smart Keymap supports low-cost MCUs from WCH (CH32X, CH58x).

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
let C = import "../ncl/codegen/contracts.ncl" in
{
  gpio_pins | { _ | C.GpioPin },

  board | C.Board = {
    usb = {
      vid = 0xCAFE,
      pid = 0x0005,
      manufacturer = "rgoulter keyboard-labs",
      product = "Pico42"
    },

    matrix =
      let p = gpio_pins in
      {
        cols = [p.GP0, p.GP1, p.GP2, p.GP3, p.GP4, p.GP5, p.GP6, p.GP7, p.GP8, p.GP9, p.GP10, p.GP11,],
        rows = [p.GP14, p.GP15, p.GP16, p.GP17,],
        key_count = 42,
      },

    keymap_index_for_key = fun { column_index, row_index } =>
      let [
        k00, k01, k02, k03, k04, k05, k06, k07, k08, k09,
        k10, k11, k12, k13, k14, k15, k16, k17, k18, k19,
        k20, k21, k22, k23, k24, k25, k26, k27, k28, k29,
        k30, k31, k32, k33, k34, k35, k36, k37, k38, k39,
        k40, k41,
      ] = std.array.generate (fun i => 'Key i) 42 in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03, k04,  NO,  NO, k05, k06, k07, k08, k09],
        [k10, k11, k12, k13, k14,  NO,  NO, k15, k16, k17, k18, k19],
        [k20, k21, k22, k23, k24,  NO,  NO, k25, k26, k27, k28, k29],
        [k30, k31, k32, k33, k34, k35, k36, k37, k38, k39, k40, k41],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,
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

### STM32F4 RTIC & Embassy Keyboard firmware

Firmware for [MiniF4-36 rev2021.4](https://github.com/rgoulter/keyboard-labs#x-1-36-key-split-reversible-pcb-with-weact-studio-mini-f4-dev-board)
split halves are implemented under [stm32f4-rtic-smart-keyboard/examples](stm32f4-rtic-smart-keyboard/examples).

Firmware for [MiniF4-36 rev2021.5](https://github.com/rgoulter/keyboard-labs#x-1-36-key-split-reversible-pcb-with-weact-studio-mini-f4-dev-board)
split halves are implemented under [stm32-embassy-smart-keyboard/examples](stm32-embassy-smart-keyboard/examples).

The `memory.x` used to link the firmware assumes [tinyuf2](https://github.com/adafruit/tinyuf2) is used as the bootloader.

The STM32F4-RTIC examples can be built by running:

```
make minif4_36-rev2021_4-lhs.uf2
make minif4_36-rev2021_4-rhs.uf2
```

The keymap file used can be changed by setting `SMART_KEYMAP_CUSTOM_KEYMAP` in the `make` invocation:

```
make minif4_36-rev2021_4-lhs.uf2 SMART_KEYMAP_CUSTOM_KEYMAP=path/to/keymap.ncl
```

### CH32X035 USB HID Keyboard

The example USB HID keyboard software from the EVT has been adapted
to work with smart_keymap lib.

See [firmware/ch32x035-usb-device-compositekm-c](firmware/ch32x035-usb-device-compositekm-c/README.MD) for more details.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
