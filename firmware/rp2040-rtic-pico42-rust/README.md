There is keyboard firmware written in Rust using the [keyberon](https://github.com/TeXitoi/keyberon)
firmware.

## Compiling

## Using `nix`

```shell
nix build .#firmware-uf2
```

puts the `.uf2` files for all the firmware (in `src/bin`) in `result/bin/`.

## Using `cargo`

Compile the firmware:

```shell
cargo objcopy --bin <firmware src> --release -- -O binary firmware.bin
```

where `<firmware src>` is one of `src/bin/<firmware src>.rs`.

## Flashing

#### Using UF2

I recommend flashing [tinyuf2](https://github.com/adafruit/tinyuf2)
onto the dev board. This makes flashing the firmware as easy
as copying the file onto a flashdrive.
