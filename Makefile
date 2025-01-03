CARGO = cargo
CBINDGEN = cbindgen

.PHONY: all
all: generate-header
	$(CARGO) build

.PHONY: generate-header
generate-header: include/smart_keymap.h

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

.PHONY: rv-check60
rv-check60:
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/keymaps/checkkeys_60key_keymap.rs" \
	  $(CARGO) build \
	    --target riscv32imac-unknown-none-elf \
	    --release \
		--no-default-features \
	    --features "usbd-human-interface-device"

include/smart_keymap.h:
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
