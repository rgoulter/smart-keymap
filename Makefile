CARGO = cargo
CBINDGEN = cbindgen

RP2040_TARGET := thumbv6m-none-eabi
STM32F4_TARGET := thumbv7em-none-eabihf

STM32F4_TARGET_DIR := target/$(STM32F4_TARGET)
STM32F4_DEBUG_TARGET_DIR ?= $(STM32F4_TARGET_DIR)/debug
STM32F4_RELEASE_TARGET_DIR ?= $(STM32F4_TARGET_DIR)/release

DEST_DIR ?= .

STM32F4_RTIC_EXAMPLES := $(notdir $(basename $(wildcard stm32f4-rtic-smart-keyboard/examples/*.rs)))
STM32F4_EMBASSY_EXAMPLES := $(notdir $(basename $(wildcard stm32f4-embassy-smart-keyboard/examples/*.rs)))
STM32F4_EXAMPLES := $(STM32F4_RTIC_EXAMPLES) $(STM32F4_EMBASSY_EXAMPLES )

EXAMPLES := $(STM32F4_EXAMPLES)
EXAMPLES_BIN := $(addprefix example-, $(addsuffix .bin,$(EXAMPLES)))
EXAMPLES_UF2 := $(addprefix example-, $(addsuffix .uf2,$(EXAMPLES)))

TARGETS_BIN = $(BINARIES_BIN) $(EXAMPLES_BIN)
TARGETS_UF2 = $(BINARIES_UF2) $(EXAMPLES_UF2)

export SMART_KEYMAP_CUSTOM_KEYMAP

ifndef VERBOSE
MAKEFLAGS += --no-print-directory
endif

include ncl/ncl.mk
include tests/ceedling/ceedling.mk

.PHONY: all
all: include/smart_keymap.h
	$(CARGO) build

.PHONY: test
# Run the NCL checks (and NCL codegen snapshot checks)
test: test-ncl
# Run the rust tests (incl. integration)
test: test-rust
# Run the Ceedling integration tests
test: test-ceedling
# Run rust tidyness checks
test: test-clippy
test: test-doc
test: test-fmt
# Check the rust firmware builds
test: build-rust-stm32f4
test: build-rust-thumbv6m-none-eabi

.PHONY: test-rust
test-rust:
	$(CARGO) test

# Match .github/workflows/rust.yaml cargo-fmt-clippy job.
.PHONY: test-fmt
test-fmt:
	$(CARGO) fmt --all -- --check

.PHONY: test-clippy
test-clippy:
	$(CARGO) clippy --workspace \
		--exclude rp2040-rtic-smart-keyboard \
		--exclude stm32f4-rtic-smart-keyboard \
		--exclude stm32-embassy-smart-keyboard \
		-- -D warnings

# Match CI cargo-doc steps (host + firmware packages).
# Host: .github/workflows/rust.yaml
# Firmware: rust-stm32f4.yaml, rust-thumbv6m-none-eabi.yaml
.PHONY: test-doc
test-doc: test-doc-host test-doc-firmware

.PHONY: test-doc-host
test-doc-host:
	RUSTDOCFLAGS="--deny warnings" $(CARGO) doc \
		--package=smart-keymap-core \
		--package=smart-keymap \
		--package=keyberon-smart-keyboard

.PHONY: test-doc-firmware
test-doc-firmware: test-doc-stm32f4 test-doc-rp2040

# Match .github/workflows/rust-stm32f4.yaml "Run Cargo Doc"
.PHONY: test-doc-stm32f4
test-doc-stm32f4:
	RUSTDOCFLAGS="--deny warnings" $(CARGO) doc \
		--no-default-features \
		--package=stm32f4-rtic-smart-keyboard \
		--package=smart-keymap \
		--package=keyberon-smart-keyboard

# Match .github/workflows/rust-thumbv6m-none-eabi.yaml "Run Cargo Doc"
.PHONY: test-doc-rp2040
test-doc-rp2040:
	RUSTDOCFLAGS="--deny warnings" $(CARGO) doc \
		--package=rp2040-rtic-smart-keyboard \
		--package=smart-keymap \
		--package=keyberon-smart-keyboard

.PHONY: build-rust-thumbv6m-none-eabi
build-rust-thumbv6m-none-eabi:
	$(CARGO) build --release --target=$(RP2040_TARGET) --no-default-features
	$(CARGO) build --release --target=$(RP2040_TARGET) --package=keyberon-smart-keyboard
	$(CARGO) build --release --target=$(RP2040_TARGET) --package=rp2040-rtic-smart-keyboard

.PHONY: build-rust-rp2040
build-rust-rp2040: build-rust-thumbv6m-none-eabi

.PHONY: build-rust-thumbv7em-none-eabihf
build-rust-thumbv7em-none-eabihf:
	$(CARGO) build --release --target=$(STM32F4_TARGET) --no-default-features
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=keyberon-smart-keyboard
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-lhs
	$(CARGO) build --release --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --example=minif4_36-rev2021_4-rhs

.PHONY: build-rust-stm32f4
build-rust-stm32f4: build-rust-thumbv7em-none-eabihf

.PHONY: clean
clean: clean-generated-keymaps clean.bin clean.uf2
	rm -f include/smart_keymap.h
	$(CARGO) clean

include/smart_keymap.h: smart_keymap/src/lib.rs src/lib.rs
	$(CBINDGEN) -c cbindgen.toml -o include/smart_keymap.h ./smart_keymap

.PHONY: clean.bin
clean.bin:
	rm -f $(addprefix $(DEST_DIR)/,$(TARGETS_BIN))

.PHONY: clean.uf2
clean.uf2:
	rm -f $(addprefix $(DEST_DIR)/,$(TARGETS_UF2))

.PHONY: targets.bin
targets.bin: $(addprefix $(DEST_DIR)/,$(TARGETS_BIN))

.PHONY: targets.uf2
targets.uf2: $(addprefix $(DEST_DIR)/,$(TARGETS_UF2))

$(STM32F4_RELEASE_TARGET_DIR)/examples/minif4_36-rev2021_4-lhs: SMART_KEYMAP_CUSTOM_KEYMAP=$(shell pwd)/tests/ncl/keymap-36key-rgoulter/keymap.ncl

$(STM32F4_RELEASE_TARGET_DIR)/examples/minif4_36-rev2021_4-rhs: SMART_KEYMAP_CUSTOM_KEYMAP=$(shell pwd)/tests/ncl/keymap-36key-rgoulter/keymap.ncl

$(STM32F4_RELEASE_TARGET_DIR)/examples/minif4_36-rev2021_5-lhs: SMART_KEYMAP_CUSTOM_KEYMAP=$(shell pwd)/tests/ncl/keymap-36key-rgoulter/keymap.ncl

$(STM32F4_RELEASE_TARGET_DIR)/examples/minif4_36-rev2021_5-rhs: SMART_KEYMAP_CUSTOM_KEYMAP=$(shell pwd)/tests/ncl/keymap-36key-rgoulter/keymap.ncl

$(STM32F4_RELEASE_TARGET_DIR)/examples/%: stm32f4-rtic-smart-keyboard/examples/%.rs
	cargo build --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --release --example="$*"

$(STM32F4_RELEASE_TARGET_DIR)/%: stm32f4-rtic-smart-keyboard/src/bin/%.rs
	cargo build --target=$(STM32F4_TARGET) --package=stm32f4-rtic-smart-keyboard --release --bin="$*"

$(STM32F4_RELEASE_TARGET_DIR)/examples/%: stm32-embassy-smart-keyboard/examples/%.rs
	cargo build --target=$(STM32F4_TARGET) --package=stm32-embassy-smart-keyboard --release --example="$*"

$(DEST_DIR)/%.bin: $(STM32F4_RELEASE_TARGET_DIR)/examples/%
	rust-objcopy $(STM32F4_RELEASE_TARGET_DIR)/examples/$* --output-target "binary" $(DEST_DIR)/$*.bin

$(DEST_DIR)/%.bin: $(STM32F4_RELEASE_TARGET_DIR)/%
	rust-objcopy $(STM32F4_RELEASE_TARGET_DIR)/$* --output-target "binary" $(DEST_DIR)/$*.bin

$(DEST_DIR)/%.uf2: $(DEST_DIR)/%.bin $(STM32F4_RELEASE_TARGET_DIR)/examples/%
	uf2conv --convert --family=STM32F4 --base 0x8010000 --output="$(DEST_DIR)/$*.uf2" $(DEST_DIR)/$*.bin

$(DEST_DIR)/%.uf2: $(DEST_DIR)/%.bin $(STM32F4_RELEASE_TARGET_DIR)/%
	uf2conv --convert --family=STM32F4 --base 0x8010000 --output="$(DEST_DIR)/$*.uf2" $(DEST_DIR)/$*.bin
