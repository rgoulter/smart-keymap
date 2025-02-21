BOARD=ncl/wabble-60.ncl

.PHONY: all
all: generated/matrix.c libsmartkeymap

.PHONY: clean
clean:
	rm generated/matrix.c

.PHONY: test
test:
	nickel \
	  eval \
	  --field=keymap_indices \
	  ncl/wabble-60.ncl \
	  ncl/test-expected-wabble-60.ncl \
	  ncl/test-actual-wabble-60.ncl

.PHONY: generated/matrix.c
generated/matrix.c:
	nickel export \
	  --format=raw \
	  --field=output \
	  $(BOARD) \
	  ncl/matrix_scan.ncl \
	  > generated/matrix.c

.PHONY: generated-wabble-60
generated-wabble-60: BOARD=ncl/wabble-60.ncl
generated-wabble-60: generated/matrix.c

.PHONY: libsmartkeymap
libsmartkeymap: libsmartkeymap/smart_keymap.h libsmartkeymap/libsmart_keymap.a;

libsmartkeymap/smart_keymap.h:
	echo "ERROR: smart_keymap.h not installed. Please copy from the smart-keymap project."

libsmartkeymap/libsmart_keymap.a:
	echo "ERROR: libsmart_keymap.a not installed. Please copy from the smart-keymap project."
