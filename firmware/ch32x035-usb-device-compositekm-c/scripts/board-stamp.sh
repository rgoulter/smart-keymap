#!/usr/bin/env sh

set -e

STAMP_FILE="$1"
CURRENT_BOARD="$2"

if [ ! -f "${STAMP_FILE}" ] || [ "$(cat "${STAMP_FILE}")" != "${CURRENT_BOARD}" ]; then
  echo "BOARD variable changed (or stamp missing). Updating stamp file for ${CURRENT_BOARD}..."
  echo "${CURRENT_BOARD}" > "${STAMP_FILE}"
else
  echo "BOARD variable (${CURRENT_BOARD}) unchanged."
fi
