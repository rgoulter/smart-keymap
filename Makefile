CARGO = cargo
CBINDGEN = cbindgen

include ncl/ncl.mk
include tests/ceedling/ceedling.mk

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
test: include/smart_keymap.h

.PHONY: clean
clean:
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
