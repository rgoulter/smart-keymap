CARGO = cargo
CBINDGEN = cbindgen

ifndef VERBOSE
MAKEFLAGS += --no-print-directory
endif

include ncl/ncl.mk
include tests/ceedling/ceedling.mk

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
test: test-rust test-ncl test-ceedling build-rust-thumbv6m-none-eabi

.PHONY: test-rust
test-rust:
	$(CARGO) test

.PHONY: build-rust-thumbv6m-none-eabi
build-rust-thumbv6m-none-eabi:
	$(CARGO) build --target=thumbv6m-none-eabi --no-default-features
	$(CARGO) build --target=thumbv6m-none-eabi --package=usbd-smart-keyboard
	$(CARGO) build --target=thumbv6m-none-eabi --package=rp2040-rtic-smart-keyboard

.PHONY: build-rust-rp2040
build-rust-rp2040: build-rust-thumbv6m-none-eabi

.PHONY: build-rust-thumbv7em-none-eabihf
build-rust-thumbv7em-none-eabihf:
	$(CARGO) build --target=thumbv7em-none-eabihf --no-default-features
	$(CARGO) build --target=thumbv7em-none-eabihf --package=usbd-smart-keyboard
	$(CARGO) build --target=thumbv7em-none-eabihf --package=stm32f4-rtic-smart-keyboard
	$(CARGO) build --target=thumbv7em-none-eabihf --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-lhs
	$(CARGO) build --target=thumbv7em-none-eabihf --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-rhs

.PHONY: build-rust-stm32f4
build-rust-stm32f4: build-rust-thumbv7em-none-eabihf

.PHONY: clean
clean: clean-generated-keymaps
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap
