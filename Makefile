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

.PHONY: clean
clean: clean-generated-keymaps
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap
