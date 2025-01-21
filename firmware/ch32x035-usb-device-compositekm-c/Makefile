.PHONY: all
all: generated/matrix.c

.PHONY: clean
clean:
	rm generated/matrix.c

.PHONY: test
test:
	nickel \
	  eval \
	  --field=keymap_indices \
	  ncl/ch32x-48.ncl \
	  ncl/test-expected-ch32x-48.ncl \
	  ncl/test-actual-ch32x-48.ncl

generated/matrix.c:
	nickel export \
	  --format=raw \
	  --field=output \
	  ncl/weact-ch32x-core-board.ncl \
	  ncl/matrix_scan.ncl \
	  > generated/matrix.c

.PHONY: generated-ch32x-48
generated-ch32x-48:
	nickel export \
	  --format=raw \
	  --field=output \
	  ncl/ch32x-48.ncl \
	  ncl/matrix_scan.ncl \
	  > generated/matrix.c
