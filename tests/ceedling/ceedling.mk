CEEDLING = ceedling

TEST_KEYMAP = keymap-4key-simple

# Ceedling vendors Unity from the Nix store with read-only perms;
# a second run fails when it tries to overwrite build/vendor/*.c without this.
.PHONY: fix-ceedling-vendor
fix-ceedling-vendor:
	if test -d tests/ceedling/build/vendor; then chmod -R u+w tests/ceedling/build/vendor; fi

.PHONY: format-ceedling
format-ceedling:
	find tests/ceedling/test -name '*.c' | xargs clang-format -i

.PHONY: test-ceedling
test-ceedling: include/smart_keymap.h fix-ceedling-vendor format-ceedling
	env SMART_KEYMAP_CUSTOM_KEYMAP="$(shell pwd)/tests/ncl/$(TEST_KEYMAP)/keymap.ncl" \
	  $(CARGO) build --package "smart_keymap"
	cd tests/ceedling && $(CEEDLING)
