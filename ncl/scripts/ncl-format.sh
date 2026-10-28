#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$root"

# nickel format 1.16 cannot parse match-or patterns in this file.
readonly SKIP=(ncl/layered-key.ncl)

is_skipped() {
  local f="$1" s
  for s in "${SKIP[@]}"; do
    [[ "$f" == "$s" ]] && return 0
  done
  return 1
}

files=()
while IFS= read -r f; do
  [[ -n "$f" ]] && ! is_skipped "$f" && files+=("$f")
done < <("${root}/ncl/scripts/ncl-format-files.sh" "$@")

if [[ ${#files[@]} -eq 0 ]]; then
  exit 0
fi

failures=()
for f in "${files[@]}"; do
  if ! nickel format "$f"; then
    failures+=("$f")
  fi
done

if [[ ${#failures[@]} -gt 0 ]]; then
  printf 'nickel format failed for: %s\n' "${failures[*]}" >&2
  exit 1
fi
