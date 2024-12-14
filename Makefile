CARGO = cargo
MESON = meson
BUILDDIR = build
CBINDGEN = cbindgen
UNITY_BASE_URI = https://raw.githubusercontent.com/ThrowTheSwitch/Unity/refs/tags/v2.6.0

.PHONY: all
all: generate-header tests/c_tests/unity/unity.h
	$(CARGO) build --features "std"
	$(MESON) setup $(BUILDDIR)
	$(MESON) compile -C $(BUILDDIR)

.PHONY: generate-header
generate-header: include/smart_keymap.h

.PHONY: test
test: include/smart_keymap.h tests/c_tests/unity/unity.h
	$(CARGO) test --features "std"
	$(MESON) setup $(BUILDDIR) || $(MESON) setup $(BUILDDIR)
	$(MESON) test -C $(BUILDDIR)

.PHONY: clean
clean:
	$(MESON) clean
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h:
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h

tests/c_tests/unity/unity.h tests/c_tests/unity/unity.c:
	mkdir -p tests/c_tests/unity
	wget $(UNITY_BASE_URI)/src/unity.c -O tests/c_tests/unity/unity.c
	wget $(UNITY_BASE_URI)/src/unity.h -O tests/c_tests/unity/unity.h
	wget $(UNITY_BASE_URI)/src/unity_internals.h -O tests/c_tests/unity/unity_internals.h
