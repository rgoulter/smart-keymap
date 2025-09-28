#!/usr/bin/env bash

set -e

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."

FEATURES_DIR="${REPOSITORY_DIR}/features"
KEYMAP_FEATURES_DIR="${FEATURES_DIR}/keymap"

NCL_DIR="${REPOSITORY_DIR}/ncl"

KEYMAP_FEATURE_MD=""

keymap_key_features=(
    "keyboard"
    "tap_hold"
    "tap_hold-config-timeout"
    "tap_hold-config-interrupt-ignore"
    "tap_hold-config-interrupt-presses"
    "tap_hold-config-interrupt-tap"
    "tap_hold-config-required_idle_time"
    "layered"
    "layer_modifier-set_active_layers"
    "layer_modifier-default"
    "caps_word"
    "tap_dance"
    "sticky_modifiers"
    "sticky_modifiers-config-release_on_next_press"
)

keymap_ncl_features=(
    "layers"
    "layer_string"
    "chords"
)

KEY_FIELDS_MD="${NCL_DIR}/key-docs.md"
nickel export \
    --field=keys_md \
    --format=text \
    --import-path="${NCL_DIR}" \
    "${NCL_DIR}/key-docs.ncl" \
    > "${KEY_FIELDS_MD}"

KEYMAP_KEY_FEATURES_DIR="${KEYMAP_FEATURES_DIR}/key"
KEYMAP_FEATURE_MD="${KEYMAP_FEATURE_MD} ${KEYMAP_KEY_FEATURES_DIR}/readme.md"
for keymap_feature in "${keymap_key_features[@]}"
do
    MD_FILE="${KEYMAP_KEY_FEATURES_DIR}/generated-${keymap_feature}.md"
    gawk --file="${SCRIPTS_DIR}/gherkin2md.awk" -- \
      "${KEYMAP_KEY_FEATURES_DIR}/${keymap_feature}.feature" \
      >  "${MD_FILE}"

    KEYMAP_FEATURE_MD="${KEYMAP_FEATURE_MD} ${MD_FILE}"
done

KEYMAP_NCL_FEATURES_DIR="${KEYMAP_FEATURES_DIR}/ncl"
KEYMAP_FEATURE_MD="${KEYMAP_FEATURE_MD} ${KEYMAP_NCL_FEATURES_DIR}/readme.md"
for keymap_feature in "${keymap_ncl_features[@]}"
do
    MD_FILE="${KEYMAP_NCL_FEATURES_DIR}/generated-${keymap_feature}.md"
    gawk --file="${SCRIPTS_DIR}/gherkin2md.awk" -- \
      "${KEYMAP_NCL_FEATURES_DIR}/${keymap_feature}.feature" \
      >  "${MD_FILE}"

    KEYMAP_FEATURE_MD="${KEYMAP_FEATURE_MD} ${MD_FILE}"
done

pandoc \
  --standalone \
  --table-of-contents \
  --embed-resources \
  --css="${FEATURES_DIR}/pandoc.css" \
  --toc-depth=4 \
  --metadata=title="Smart Keymap Features" \
  ${KEYMAP_FEATURE_MD} \
  ${KEY_FIELDS_MD} \
  --output="${FEATURES_DIR}/generated-features.html"
