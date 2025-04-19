CARGO = cargo
CBINDGEN = cbindgen

RP2040_TARGET := thumbv6m-none-eabi
STM32F4_TARGET := thumbv7em-none-eabihf

ifndef VERBOSE
MAKEFLAGS += --no-print-directory
endif

include ncl/ncl.mk
include tests/ceedling/ceedling.mk

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
test: test-rust test-ncl test-ceedling build-rust-thumbv6m-none-eabi build-rust-stm32f4

.PHONY: test-rust
test-rust:
	$(CARGO) test

.PHONY: build-rust-thumbv6m-none-eabi
build-rust-thumbv6m-none-eabi:
	$(CARGO) build --release --target=$(RP2040_TARGET) --no-default-features
	$(CARGO) build --release --target=$(RP2040_TARGET) --package=usbd-smart-keyboard
	$(CARGO) build --release --target=$(RP2040_TARGET) --package=rp2040-rtic-smart-keyboard

.PHONY: build-rust-rp2040
build-rust-rp2040: build-rust-thumbv6m-none-eabi

.PHONY: build-rust-thumbv7em-none-eabihf
build-rust-thumbv7em-none-eabihf:
	$(CARGO) build --release --target=$(STM32F4_TARGET) --no-default-features
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=usbd-smart-keyboard
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-lhs
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-rhs

.PHONY: build-rust-stm32f4
build-rust-stm32f4: build-rust-thumbv7em-none-eabihf

.PHONY: clean
clean: clean-generated-keymaps
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap
