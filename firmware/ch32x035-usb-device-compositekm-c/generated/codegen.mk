CODEGEN_DEPS = \
  $(BOARD) \
  ncl/codegen.ncl \
  ncl/codegen_keyboard.ncl

NICKEL_QUERY_CMD := \
  nickel export \
    --field=source_filenames \
    --format=raw \
    $(CODEGEN_DEPS) \
    ncl/codegen.ncl
IDS := $(shell $(NICKEL_QUERY_CMD))

CODEGEN_SOURCE_TARGETS := $(patsubst %,generated/%.c,$(IDS))

CODEGEN_TARGETS := $(CODEGEN_SOURCE_TARGETS)

generated/%.c: ncl/codegen_%.ncl $(CODEGEN_DEPS)
	nickel export \
	  --format=raw \
	  --field=sources.$* \
	  $(CODEGEN_DEPS) \
	  > $@
