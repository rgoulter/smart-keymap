.PHONY: all
all: generated/matrix.c 

.PHONY: clean
clean:
	rm generated/matrix.c

generated/matrix.c:
	nickel export \
	  --format=raw \
	  --field=output \
	  ncl/wabble-60.ncl \
	  ncl/matrix_scan.ncl \
	  > generated/matrix.c
