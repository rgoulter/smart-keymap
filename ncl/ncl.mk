.PHONY: clean-generated-keymaps
clean-generated-keymaps:
	ncl/scripts/clean-generated-keymaps.sh

.PHONY: test-ncl
test-ncl: test-ncl-checks
	ncl/scripts/run-tests.sh

.PHONY: test-ncl-checks
test-ncl-checks:
	ncl/scripts/run-ncl-checks.sh

%/keymap.json:
	ncl/scripts/keymap-ncl-to-json.sh $(shell dirname $@)

%/keymap.rs: %/keymap.json
	ncl/scripts/keymap-codegen.sh $(shell dirname $@)

# Some NCL code uses hexadecimal representation (0x00)
#  which is not supported by `nickel format`.
#
# uses hex-formatted numbers:
#   ncl/hid-usage-keyboard.ncl
.PHONY: ncl-format
ncl-format:
	nickel format \
	   ncl/smart_keys/**/*.ncl \
	   ncl/layouts/remap.ncl \
	   ncl/layouts/remap-36keys.ncl \
	   ncl/checks.ncl \
	   ncl/hid-report.ncl \
	   ncl/import-keymap-json.ncl \
	   ncl/inputs-to-json.ncl \
	   ncl/inputs.ncl \
       ncl/key-docs.ncl  \
       ncl/key-extensions.ncl  \
       ncl/keymap-ncl-to-json.ncl \
	   ncl/keymap-codegen.ncl \
	   ncl/keys.ncl \
       ncl/smart-keys.ncl  \
	   ncl/layered-key.ncl \
	   ncl/validators.ncl
