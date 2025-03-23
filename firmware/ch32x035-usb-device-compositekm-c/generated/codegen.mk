CODEGEN_DEPS = \
  $(BOARD) \
  ncl/codegen/debug.ncl \
  ncl/codegen/gpio.ncl \
  ncl/codegen/keyboard.ncl \
  ncl/codegen/keyboard_led.ncl \
  ncl/codegen/keyboard_matrix.ncl \
  ncl/codegen/keyboard_split.ncl

NICKEL_QUERY_CMAKELISTS_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=cmakelists_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl

NICKEL_QUERY_INCLUDES_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=includes_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl

NICKEL_QUERY_SOURCES_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=source_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl

CMAKELISTS_IDS := $(shell $(NICKEL_QUERY_CMAKELISTS_CMD))
INCLUDES_IDS := $(shell $(NICKEL_QUERY_INCLUDES_CMD))
SOURCE_IDS := $(shell $(NICKEL_QUERY_SOURCES_CMD))

CODEGEN_CMAKELISTS_TARGETS := $(patsubst %,generated/%.cmake,$(CMAKELISTS_IDS))
CODEGEN_INCLUDES_TARGETS := $(patsubst %,generated/%.h,$(INCLUDES_IDS))
CODEGEN_SOURCE_TARGETS := $(patsubst %,generated/%.c,$(SOURCE_IDS))

CODEGEN_TARGETS := \
	$(CODEGEN_CMAKELISTS_TARGETS) \
	$(CODEGEN_INCLUDES_TARGETS) \
	$(CODEGEN_SOURCE_TARGETS)

.PHONY: .clean-codegen
.clean-codegen:
	rm -f generated/*.cmake
	rm -f generated/keyboard.c
	rm -f generated/keyboard_led.c
	rm -f generated/keyboard_led.h
	rm -f generated/keyboard_matrix.c
	rm -f generated/keyboard_matrix.h
	rm -f generated/keyboard_split.h

.PHONY: FORCE_STAMP

generated/.board.stamp: FORCE_STAMP
	@scripts/board-stamp.sh "$@" "$(BOARD)"

generated/%.cmake: ncl/codegen/%.ncl $(CODEGEN_DEPS) generated/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=cmakelists.$* \
	  $(CODEGEN_DEPS) \
	  > $@

generated/%.h: ncl/codegen/%.ncl $(CODEGEN_DEPS) generated/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=includes.$* \
	  $(CODEGEN_DEPS) \
	  > $@

generated/%.c: ncl/codegen/%.ncl $(CODEGEN_DEPS) generated/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=sources.$* \
	  $(CODEGEN_DEPS) \
	  > $@
