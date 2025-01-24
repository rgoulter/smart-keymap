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
test: test-rust test-ncl test-ceedling

.PHONY: test-rust
test-rust:
	$(CARGO) test

.PHONY: clean
clean: clean-generated-keymaps
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
