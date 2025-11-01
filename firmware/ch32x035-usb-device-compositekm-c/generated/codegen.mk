CODEGEN_DIR := generated
NCL_DIR := ncl/codegen

CODEGEN_DEPS = \
  $(BOARD) \
  $(NCL_DIR)/debug.ncl \
  $(NCL_DIR)/gpio.ncl \
  $(NCL_DIR)/keyboard.ncl \
  $(NCL_DIR)/keyboard_led.ncl \
  $(NCL_DIR)/keyboard_matrix.ncl \
  $(NCL_DIR)/keyboard_split.ncl

NICKEL_QUERY_CMAKELISTS_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=cmakelists_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    $(NCL_DIR).ncl

NICKEL_QUERY_INCLUDES_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=includes_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    $(NCL_DIR).ncl

NICKEL_QUERY_SOURCES_CMD := \
  nickel export \
    --import-path=ncl/ \
    --field=source_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    $(NCL_DIR).ncl

CMAKELISTS_IDS := $(shell $(NICKEL_QUERY_CMAKELISTS_CMD))
INCLUDES_IDS := $(shell $(NICKEL_QUERY_INCLUDES_CMD))
SOURCE_IDS := $(shell $(NICKEL_QUERY_SOURCES_CMD))

CODEGEN_CMAKELISTS_TARGETS := $(patsubst %,$(CODEGEN_DIR)/%.cmake,$(CMAKELISTS_IDS))
CODEGEN_INCLUDES_TARGETS := $(patsubst %,$(CODEGEN_DIR)/%.h,$(INCLUDES_IDS))
CODEGEN_SOURCE_TARGETS := $(patsubst %,$(CODEGEN_DIR)/%.c,$(SOURCE_IDS))

CODEGEN_TARGETS := \
	$(CODEGEN_CMAKELISTS_TARGETS) \
	$(CODEGEN_INCLUDES_TARGETS) \
	$(CODEGEN_SOURCE_TARGETS)

.PHONY: .clean-codegen
.clean-codegen:
	rm -f $(CODEGEN_DIR)/*.cmake
	rm -f $(CODEGEN_DIR)/keyboard.c
	rm -f $(CODEGEN_DIR)/keyboard_led.c
	rm -f $(CODEGEN_DIR)/keyboard_led.h
	rm -f $(CODEGEN_DIR)/keyboard_matrix.c
	rm -f $(CODEGEN_DIR)/keyboard_matrix.h
	rm -f $(CODEGEN_DIR)/keyboard_split.h

.PHONY: FORCE_STAMP

$(CODEGEN_DIR)/.board.stamp: FORCE_STAMP
	@scripts/board-stamp.sh "$@" "$(BOARD)"

$(CODEGEN_DIR)/%.cmake: $(NCL_DIR)/%.ncl $(CODEGEN_DEPS) $(CODEGEN_DIR)/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=cmakelists.$* \
	  $(CODEGEN_DEPS) \
	  > $@

$(CODEGEN_DIR)/%.h: $(NCL_DIR)/%.ncl $(CODEGEN_DEPS) $(CODEGEN_DIR)/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=includes.$* \
	  $(CODEGEN_DEPS) \
	  > $@

$(CODEGEN_DIR)/%.c: $(NCL_DIR)/%.ncl $(CODEGEN_DEPS) $(CODEGEN_DIR)/.board.stamp
	@echo "Generating $@"
	@nickel export \
    --import-path=ncl/ \
	  --format=raw \
	  --field=sources.$* \
	  $(CODEGEN_DEPS) \
	  > $@
