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

The task-runner [just](https://just.systems/man/en/) is used
 for convenient invocation of build tasks.

The `smart-keymap`'s justfile provides an `install` recipe
 for conveniently building & installing the `libsmart_keymap.a`.

This directory's `justfile` provides a `flash` recipe for conveniently
 building & flashing the HID keyboard firmware.

From the `smart-keymap` root directory,
the firmware can be flashed with the keymap `tests/ncl/{test_keymap}/keymap.ncl`
by running e.g.:

``` sh
just \
  test_keymap=keymap-48key-checkkeys \
  dest_dir="firmware/ch32x035-usb-device-compositekm-c/libsmartkeymap/" \
  install && \
just firmware/ch32x035-usb-device-compositekm-c/flash
```

### Flashing the CH32X

If the CH32X does not have code protection enabled, `wchisp` can be used to flash
the MCU by shorting DOWNLOAD and plugging in the USB connector.

The WeAct core board, for some reason, has code protection enabled. This requires `wlink` and a
and a WCH link compatible with the CH32X035, such as WCH-LinkE.
(SEE: https://github.com/ch32-rs/wchisp/issues/68#issuecomment-2558167356).

