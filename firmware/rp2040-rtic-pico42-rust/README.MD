This firmware has been adapted from the [rgoulter/keyboard-labs firmware/keyberon](https://github.com/rgoulter/keyboard-labs/tree/master/firmware/keyberon) code.

The firmware uses

- [RTIC](https://rtic.rs/1/book/en/) as the concurrency framework for the RP2040's Cortex processor.

- [keyberon](https://github.com/TeXitoi/keyberon) for matrix scanning and debouncing.

  - The keyboard layout and "keyboard backend" component have been replaced with smart keymap.

- [usbd-human-interface-device](https://github.com/dlkj/usbd-human-interface-device) is used
 for the keyboard USB device.

In particular, the firmware sample is for the
[Pico42](https://github.com/rgoulter/keyboard-labs/releases/tag/pico42-rev2023.2).

# Building

## Rust Dependencies

[DevEnv](https://devenv.sh/) is used to provide the toolchain dependencies.

Use `devenv shell` to enter a shell which has all the tooling installed.

Otherwise, other than `rust`/`cargo`, note that it's useful to have
[elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs) in order to invoke `cargo run`.

## Smart Keymap Dependencies

Using a `keymap.ncl` (or the `keymap.rs` generated from its definition),
the keyboard firmware can be built with a command such as:

```
env SMART_KEYMAP_CUSTOM_KEYMAP="$(pwd)/../../tests/ncl/keymap-42key-dvorak-simple-with-tap_hold/keymap.ncl" \
  cargo build \
    --release \
    --target=thumbv6m-none-eabi \
    --package=rp2040-rtic-pico42-rust-rp2040
```

The firmware can be deployed to an RP2040 board (in bootloader mode)
 by using `cargo run` instead of `cargo build`.
