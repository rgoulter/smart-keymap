#!/usr/bin/env bash

set -ex

SCRIPTS_DIR="$(dirname "$0")"
REPOSITORY_DIR="${SCRIPTS_DIR}/../.."

FEATURES_DIR="${REPOSITORY_DIR}/features"
KEYMAP_FEATURES_DIR="${FEATURES_DIR}/keymap"

keymap_features=(
    "simple"
    "tap_hold"
    "layered"
)

KEYMAP_FEATURE_MD="${KEYMAP_FEATURES_DIR}/keys.md"
for keymap_feature in "${keymap_features[@]}"
do
    gawk --file="${SCRIPTS_DIR}/gherkin2md.awk" -- \
      "${KEYMAP_FEATURES_DIR}/${keymap_feature}.feature" \
      >  "${FEATURES_DIR}/generated-${keymap_feature}.md"

    KEYMAP_FEATURE_MD="${KEYMAP_FEATURE_MD} ${FEATURES_DIR}/generated-${keymap_feature}.md"
done

pandoc \
  --standalone=true \
  --table-of-contents=true \
  --toc-depth=4 \
  --metadata=title="Smart Keymap Features" \
  ${KEYMAP_FEATURE_MD} \
  --output="${FEATURES_DIR}/generated-features.html"
