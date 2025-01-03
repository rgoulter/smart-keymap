CARGO = cargo
CBINDGEN = cbindgen

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
test: include/smart_keymap.h
	$(CARGO) test
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/keymaps/simple_keymap.rs" \
	  $(CARGO) build
	cd tests/ceedling && ceedling

.PHONY: clean
clean:
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
