.PHONY: ncl-format
ncl-format:
	nickel format \
		ncl/codegen.ncl \
		ncl/contracts.ncl \
		ncl/debug.ncl \
		ncl/gpio.ncl \
		ncl/keyboard.ncl \
		ncl/keyboard_led.ncl \
		ncl/keyboard_matrix.ncl \
		ncl/keyboard_split.ncl \
		ncl/matrix/col_to_row.ncl \
		ncl/matrix/row_to_col.ncl
