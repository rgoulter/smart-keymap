BOARD=ncl/boards/wabble-60.ncl

.PHONY: all
all: generated/keyboard.c libsmartkeymap

include generated/codegen.mk

.PHONY: clean
clean: .clean-codegen

.PHONY: test
test:
	nickel \
	  eval \
	  --import-path=ncl/ \
	  --field=keymap_indices \
	  ncl/boards/wabble-60.ncl \
	  ncl/test-expected-wabble-60.ncl \
	  ncl/test-actual-wabble-60.ncl

.PHONY: format
format:
	find APP -name '*.c' -or -name '*.h' | xargs clang-format -i
	find Profile -name '*.c' -or -name '*.h' | xargs clang-format -i

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: generated-wabble-60
generated-wabble-60: BOARD=ncl/boards/wabble-60.ncl
generated-wabble-60: codegen

.PHONY: libsmartkeymap
libsmartkeymap: libsmartkeymap/smart_keymap.h libsmartkeymap/libsmart_keymap.a;

libsmartkeymap/smart_keymap.h:
	echo "ERROR: smart_keymap.h not installed. Please copy from the smart-keymap project."

libsmartkeymap/libsmart_keymap.a:
	echo "ERROR: libsmart_keymap.a not installed. Please copy from the smart-keymap project."
