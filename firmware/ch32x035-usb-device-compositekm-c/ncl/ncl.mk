.PHONY: ncl-format
ncl-format:
	nickel format \
		ncl/boards/weact-ch32x-core-board.ncl \
		ncl/codegen/contracts.ncl \
		ncl/codegen/gpio.ncl \
		ncl/codegen/keyboard.ncl \
		ncl/codegen/keyboard_led.ncl \
		ncl/codegen/keyboard_matrix.ncl \
		ncl/codegen/matrix/col_to_row.ncl \
		ncl/codegen/matrix/row_to_col.ncl \
		ncl/codegen.ncl \
		ncl/test-actual-ch32x-48.ncl \
		ncl/test-expected-ch32x-48.ncl
