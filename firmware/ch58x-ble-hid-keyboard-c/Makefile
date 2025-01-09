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
	  ncl/wabble-60.ncl \
	  ncl/test-expected-wabble-60.ncl \
	  ncl/test-actual-wabble-60.ncl

generated/matrix.c:
	nickel export \
	  --format=raw \
	  --field=output \
	  ncl/wabble-60.ncl \
	  ncl/matrix_scan.ncl \
	  > generated/matrix.c
