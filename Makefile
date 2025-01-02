CARGO = cargo
CBINDGEN = cbindgen

.PHONY: all
all: generate-header
	$(CARGO) build --features "std"

.PHONY: generate-header
generate-header: include/smart_keymap.h

.PHONY: test
test:
	$(CARGO) clean
	$(CARGO) test --features "std"
	$(CARGO) clean
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/keymaps/simple_keymap.rs" \
	  $(CARGO) build --features "std"
	cd tests/ceedling && ceedling

.PHONY: clean
clean:
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h:
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
