BOARD=ncl/weact-ch32x-core-board.ncl

CODEGEN_DEPS = \
  $(BOARD) \
  ncl/codegen.ncl \
  ncl/codegen_keyboard.ncl

NICKEL_QUERY_CMD := \
  nickel export \
    --field=source_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl
IDS := $(shell $(NICKEL_QUERY_CMD))

CODEGEN_SOURCE_TARGETS := $(patsubst %,generated/%.c,$(IDS))

.PHONY: all
all: generated/keyboard.c libsmartkeymap

.PHONY: clean
clean:
	rm generated/keyboard.c

.PHONY: test
test:
	nickel \
	  eval \
	  --field=keymap_indices \
	  ncl/ch32x-48.ncl \
	  ncl/test-expected-ch32x-48.ncl \
	  ncl/test-actual-ch32x-48.ncl

.PHONY: format
format:
	find User -name '*.c' -or -name '*.h' | xargs clang-format -i

generated/%.c: ncl/codegen_%.ncl $(CODEGEN_DEPS)
	nickel export \
	  --format=raw \
	  --field=sources.$* \
	  $(CODEGEN_DEPS) \
	  > $@

.PHONY: generated-ch32x-48
generated-ch32x-48: BOARD=ncl/ch32x-48.ncl
generated-ch32x-48: generated/keyboard.c

.PHONY: libsmartkeymap
libsmartkeymap: libsmartkeymap/smart_keymap.h libsmartkeymap/libsmart_keymap.a;

libsmartkeymap/smart_keymap.h:
	echo "ERROR: smart_keymap.h not installed. Please copy from the smart-keymap project."

libsmartkeymap/libsmart_keymap.a:
	echo "ERROR: libsmart_keymap.a not installed. Please copy from the smart-keymap project."
