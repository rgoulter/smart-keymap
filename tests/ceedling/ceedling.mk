CEEDLING = ceedling
CEEDLING_KEYMAP_SUITES = consumer callback keyboard layered mouse sticky tap_hold remap_named_layers
CEEDLING_CARGO_TARGET = tests/ceedling/cargo-target

# Rebuild copied libs when smart_keymap crate inputs change (not only keymap.ncl).
CEEDLING_SMART_KEYMAP_RUST_DEPS = \
	Cargo.toml \
	build.rs \
	smart_keymap/Cargo.toml \
	smart_keymap/src/lib.rs \
	$(wildcard src/*.rs) \
	$(wildcard src/**/*.rs) \
	$(wildcard smart-keymap-macros/src/*.rs) \
	smart-keymap-macros/build.rs \
	smart-keymap-macros/Cargo.toml \
	$(wildcard smart-keymap-nickel-helper/src/*.rs) \
	smart-keymap-nickel-helper/Cargo.toml \
	$(wildcard ncl/*.ncl)

# Ceedling vendors Unity from the Nix store with read-only perms;
# a second run fails when it tries to overwrite build/vendor/*.c without this.
.PHONY: fix-ceedling-vendor
fix-ceedling-vendor:
	if test -d tests/ceedling/build/vendor; then chmod -R u+w tests/ceedling/build/vendor; fi

.PHONY: format-ceedling
format-ceedling:
	find tests/ceedling/test \( -name '*.c' -o -name '*.h' \) | xargs clang-format -i

CEEDLING_LIBS = \
	$(addprefix tests/ceedling/libs/libsmart_keymap_,$(addsuffix .a,$(CEEDLING_KEYMAP_SUITES))) \
	tests/ceedling/libs/libsmart_keymap_default.a
CEEDLING_FIXTURES = $(addprefix tests/ceedling/generated/,$(addsuffix _test_ceedling_fixture.h,$(CEEDLING_KEYMAP_SUITES)))

define CEEDLING_KEYMAP_SUITE_RULES
tests/ceedling/generated/$(1)_test_ceedling_fixture.h: tests/ceedling/keymaps/$(1)/keymap.ncl tests/ceedling/ncl/ceedling-fixture.ncl tests/ceedling/scripts/ceedling-fixture.sh
	mkdir -p tests/ceedling/generated
	tests/ceedling/scripts/ceedling-fixture.sh tests/ceedling/keymaps/$(1) > $$@

$(CEEDLING_CARGO_TARGET)/$(1)/debug/libsmart_keymap.a: \
		tests/ceedling/keymaps/$(1)/keymap.ncl \
		include/smart_keymap.h \
		tests/ceedling/generated/$(1)_test_ceedling_fixture.h \
		$(CEEDLING_SMART_KEYMAP_RUST_DEPS)
	mkdir -p $(CEEDLING_CARGO_TARGET)/$(1)
	env CARGO_TARGET_DIR="$(CURDIR)/$(CEEDLING_CARGO_TARGET)/$(1)" \
	  SMART_KEYMAP_CUSTOM_KEYMAP="$(CURDIR)/tests/ceedling/keymaps/$(1)/keymap.ncl" \
	  $(CARGO) build --package "smart_keymap"

tests/ceedling/libs/libsmart_keymap_$(1).a: $(CEEDLING_CARGO_TARGET)/$(1)/debug/libsmart_keymap.a
	mkdir -p tests/ceedling/libs
	cp $$< $$@
endef

$(foreach suite,$(CEEDLING_KEYMAP_SUITES),$(eval $(call CEEDLING_KEYMAP_SUITE_RULES,$(suite))))

$(CEEDLING_CARGO_TARGET)/default/debug/libsmart_keymap.a: include/smart_keymap.h $(CEEDLING_SMART_KEYMAP_RUST_DEPS)
	mkdir -p $(CEEDLING_CARGO_TARGET)/default
	env CARGO_TARGET_DIR="$(CURDIR)/$(CEEDLING_CARGO_TARGET)/default" \
	  env -u SMART_KEYMAP_CUSTOM_KEYMAP \
	  $(CARGO) build --package "smart_keymap"

tests/ceedling/libs/libsmart_keymap_default.a: $(CEEDLING_CARGO_TARGET)/default/debug/libsmart_keymap.a
	mkdir -p tests/ceedling/libs
	cp $< $@

.PHONY: test-ceedling
test-ceedling: include/smart_keymap.h
test-ceedling: fix-ceedling-vendor
test-ceedling: format-ceedling
test-ceedling: $(CEEDLING_LIBS)
test-ceedling: $(CEEDLING_FIXTURES)
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/callback.yml test:path[callback]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/consumer.yml test:path[consumer]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/keyboard.yml test:path[keyboard]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/layered.yml test:path[layered]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/mouse.yml test:path[mouse]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/sticky.yml test:path[sticky]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/tap_hold.yml test:path[tap_hold]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/remap_named_layers.yml test:path[remap_named_layers]
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/protocol.yml test:path[protocol]
