.PHONY: clean-generated-keymaps
clean-generated-keymaps:
	ncl/scripts/clean-generated-keymaps.sh

.PHONY: test-ncl
test-ncl: test-ncl-checks test-ncl-snapshots

.PHONY: test-ncl-checks
test-ncl-checks:
	ncl/scripts/run-ncl-checks.sh

.PHONY: test-ncl-snapshots
test-ncl-snapshots:
	ncl/scripts/run-snapshots.sh

%/keymap.json:
	ncl/scripts/keymap-ncl-to-json.sh $(shell dirname $@)

%/keymap.rs: %/keymap.json
	ncl/scripts/keymap-codegen.sh $(shell dirname $@)

# Whitelist: ncl/format-whitelist
.PHONY: ncl-format
ncl-format:
	ncl/scripts/ncl-format.sh
