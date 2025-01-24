CARGO = cargo
CBINDGEN = cbindgen

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

%/keymap.json:
	ncl/scripts/keymap-ncl-to-json.sh $(shell dirname $@)

%/keymap.rs: %/keymap.json
	ncl/scripts/keymap-codegen.sh $(shell dirname $@)

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
