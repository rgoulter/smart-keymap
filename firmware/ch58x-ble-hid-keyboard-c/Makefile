.PHONY: all
all: generated/matrix.c 

.PHONY: clean
clean:
	rm generated/matrix.c

generated/matrix.c:
	nickel export --format=raw ncl/matrix_scan.ncl > generated/matrix.c
