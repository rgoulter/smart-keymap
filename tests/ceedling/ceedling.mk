CEEDLING = ceedling
CEEDLING_KEYMAP_SUITES = consumer callback keyboard layered conditional_layers mouse sticky tap_hold remap_named_layers
CEEDLING_CARGO_TARGET = tests/ceedling/cargo-target

# Rebuild copied libs when smart_keymap crate inputs change (not only keymap.ncl).
CEEDLING_SMART_KEYMAP_RUST_DEPS = \
	Cargo.toml \
	build.rs \
	smart_keymap/Cargo.toml \
	smart_keymap/src/lib.rs \
	$(wildcard src/*.rs) \
	$(wildcard smart-keymap-core/src/*.rs) \
	$(wildcard smart-keymap-core/src/**/*.rs) \
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

# Suites that run under ceedling (keymap-backed + protocol).
CEEDLING_TEST_SUITES = $(CEEDLING_KEYMAP_SUITES) protocol

# Run one suite: make test-ceedling-keyboard
# Builds only that suite's lib/fixture (protocol uses the default lib).
define CEEDLING_SUITE_TEST_RULE
.PHONY: test-ceedling-$(1)
test-ceedling-$(1): include/smart_keymap.h
test-ceedling-$(1): tests/ceedling/libs/libsmart_keymap_$(1).a
test-ceedling-$(1): tests/ceedling/generated/$(1)_test_ceedling_fixture.h
	# Ceedling re-vendors Unity as read-only from the Nix store each suite;
	# chmod before every run so a multi-suite make does not hit EACCES.
	if test -d tests/ceedling/build/vendor; then chmod -R u+w tests/ceedling/build/vendor; fi
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/$(1).yml test:path[$(1)]
endef

$(foreach suite,$(CEEDLING_KEYMAP_SUITES),$(eval $(call CEEDLING_SUITE_TEST_RULE,$(suite))))

.PHONY: test-ceedling-protocol
test-ceedling-protocol: include/smart_keymap.h
test-ceedling-protocol: tests/ceedling/libs/libsmart_keymap_default.a
	if test -d tests/ceedling/build/vendor; then chmod -R u+w tests/ceedling/build/vendor; fi
	cd tests/ceedling && $(CEEDLING) --mixin=mixins/protocol.yml test:path[protocol]

.PHONY: test-ceedling
test-ceedling: include/smart_keymap.h
test-ceedling: fix-ceedling-vendor
test-ceedling: format-ceedling
test-ceedling: $(CEEDLING_LIBS)
test-ceedling: $(CEEDLING_FIXTURES)
test-ceedling: $(addprefix test-ceedling-,$(CEEDLING_TEST_SUITES))
