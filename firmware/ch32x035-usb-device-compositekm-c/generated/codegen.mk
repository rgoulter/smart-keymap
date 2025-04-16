CODEGEN_DEPS = \
  $(BOARD) \
  ncl/codegen.ncl \
  ncl/codegen_keyboard.ncl

NICKEL_QUERY_CMAKELISTS_CMD := \
  nickel export \
    --field=cmakelists_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl

NICKEL_QUERY_INCLUDES_CMD := \
  nickel export \
    --field=includes_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl

NICKEL_QUERY_SOURCES_CMD := \
  nickel export \
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

generated/%.cmake: ncl/codegen_%.ncl $(CODEGEN_DEPS)
	nickel export \
	  --format=raw \
	  --field=cmakelists.$* \
	  $(CODEGEN_DEPS) \
	  > $@

generated/%.h: ncl/codegen_%.ncl $(CODEGEN_DEPS)
	nickel export \
	  --format=raw \
	  --field=includes.$* \
	  $(CODEGEN_DEPS) \
	  > $@

generated/%.c: ncl/codegen_%.ncl $(CODEGEN_DEPS)
	nickel export \
	  --format=raw \
	  --field=sources.$* \
	  $(CODEGEN_DEPS) \
	  > $@
