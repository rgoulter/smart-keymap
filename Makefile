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

.PHONY: rv-check60
rv-check60:
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/keymaps/checkkeys_60key_keymap.rs" \
	  $(CARGO) build \
	    --target riscv32imac-unknown-none-elf \
	    --features "usbd-human-interface-device" \
	    --release

include/smart_keymap.h:
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h
