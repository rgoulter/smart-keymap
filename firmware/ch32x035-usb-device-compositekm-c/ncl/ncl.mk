.PHONY: ncl-format
ncl-format:
	nickel format \
		ncl/boards/weact-ch32x-core-board.ncl \
		ncl/test-actual-ch32x-48.ncl \
		ncl/test-expected-ch32x-48.ncl
