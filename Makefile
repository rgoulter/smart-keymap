CARGO = cargo
CBINDGEN = cbindgen

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
test: include/smart_keymap.h
	$(CARGO) test
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/ncl/keymap-4key-simple/keymap.ncl" \
	  $(CARGO) rustc --crate-type "staticlib"
	cd tests/ceedling && ceedling

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
