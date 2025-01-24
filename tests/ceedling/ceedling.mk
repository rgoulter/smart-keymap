CEEDLING = ceedling

TEST_KEYMAP = keymap-4key-simple

.PHONY: test-ceedling
test-ceedling: include/smart_keymap.h
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/ncl/$(TEST_KEYMAP)/keymap.ncl" \
	  $(CARGO) rustc --crate-type "staticlib"
	cd tests/ceedling && $(CEEDLING)
