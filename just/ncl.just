# Nickel checks, codegen snapshots, and format.
# Recipes run from the repository root.

set working-directory := ".."

# Run Nickel evaluated_checks (make test-ncl-checks)
[group('ncl')]
checks:
    make test-ncl-checks

# Diff+build all tests/ncl snapshot fixtures (make test-ncl-snapshots)
[group('ncl')]
snapshots:
    make test-ncl-snapshots

# Checks + snapshots (make test-ncl)
[group('ncl')]
all:
    make test-ncl

# Run one snapshot fixture (diff expected.rs, then cargo-check keymap)
# Example: just ncl::snapshot keymap-1key-simple
[group('ncl')]
snapshot fixture:
    ncl/scripts/test-ncl-diff.sh "{{ fixture }}"
    ncl/scripts/test-ncl-builds.sh "{{ fixture }}"

# fzf-pick a snapshot fixture and run it
[group('ncl')]
snapshot-pick:
    #!/usr/bin/env bash
    set -euo pipefail
    fixture="$(ncl/scripts/list-ncl-snapshot-fixtures.sh | fzf --prompt='ncl snapshot> ')"
    test -n "${fixture}"
    just ncl::snapshot "${fixture}"

# Refresh expected.rs for one fixture, or all if omitted
# (just ncl::save | just ncl::save keymap-1key-simple)
[group('ncl')]
save fixture="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ -z "{{ fixture }}" ]]; then
      ncl/scripts/save-test-snapshots.sh
    else
      ncl/scripts/save-keymap-rs-snapshot.sh "{{ fixture }}"
    fi

# fzf-pick a fixture and refresh its expected.rs
[group('ncl')]
save-pick:
    #!/usr/bin/env bash
    set -euo pipefail
    fixture="$(ncl/scripts/list-ncl-snapshot-fixtures.sh | fzf --prompt='ncl save> ')"
    test -n "${fixture}"
    just ncl::save "${fixture}"

# Format Nickel sources (make ncl-format)
[group('ncl')]
format:
    make ncl-format

# Format Nickel sources and fail if whitelisted files would change (CI-style)
[group('ncl')]
format-check:
    #!/usr/bin/env bash
    set -euo pipefail
    make ncl-format
    files=()
    while IFS= read -r f; do
      [[ -n "$f" ]] && files+=("$f")
    done < <(ncl/scripts/ncl-format-files.sh)
    if [[ ${#files[@]} -eq 0 ]]; then
      exit 0
    fi
    if ! git diff --exit-code -- "${files[@]}"; then
      echo "Nickel formatting changed files. Run 'just ncl::format' (or 'just fmt') and commit." >&2
      exit 1
    fi
