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
