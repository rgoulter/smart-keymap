#!/usr/bin/env bash
# List .ncl files from ncl/format-whitelist matching optional path arguments.
# With no arguments, lists every whitelisted file. With paths, lists only those
# that appear on the whitelist (for pre-commit).
set -euo pipefail

root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$root"

whitelist="${root}/ncl/format-whitelist"

to_repo_path() {
  local f="$1"
  if [[ "$f" == "$root"/* ]]; then
    printf '%s' "${f#"$root"/}"
  else
    printf '%s' "$f"
  fi
}

expand_entry() {
  local entry="$1"
  if [[ "$entry" == */ ]]; then
    find "${entry%/}" -name '*.ncl' -type f 2>/dev/null || true
  elif [[ -f "$entry" ]]; then
    printf '%s\n' "$entry"
  fi
}

matches_whitelist() {
  local f="$1"
  local entry
  while IFS= read -r entry || [[ -n "$entry" ]]; do
    [[ -z "$entry" || "$entry" == \#* ]] && continue
    if [[ "$entry" == */ ]]; then
      [[ "$f" == "${entry}"* ]] && return 0
    elif [[ "$f" == "$entry" ]]; then
      return 0
    fi
  done <"$whitelist"
  return 1
}

if [[ ! -f "$whitelist" ]]; then
  echo "missing whitelist: $whitelist" >&2
  exit 1
fi

if [[ $# -eq 0 ]]; then
  while IFS= read -r entry || [[ -n "$entry" ]]; do
    [[ -z "$entry" || "$entry" == \#* ]] && continue
    expand_entry "$entry"
  done <"$whitelist" | sort -u
  exit 0
fi

for f in "$@"; do
  f="$(to_repo_path "$f")"
  matches_whitelist "$f" && printf '%s\n' "$f"
done
