# Ceedling C/FFI integration tests.
# Prefer make test-ceedling-<suite> so lib/fixture deps stay in ceedling.mk.
# Recipes run from the repository root.

set working-directory := ".."

# All Ceedling suites (make test-ceedling; rebuilds libs, formats C)
[group('ceedling')]
all:
    make test-ceedling

# One suite by name (builds only that suite's lib/fixture when needed)
# (just ceedling::suite keyboard | just ceedling::suite protocol)
[group('ceedling')]
suite name:
    make test-ceedling-{{ name }}

# fzf-pick a suite from tests/ceedling/mixins/
[group('ceedling')]
suite-pick:
    #!/usr/bin/env bash
    set -euo pipefail
    name="$(
      find tests/ceedling/mixins -maxdepth 1 -name '*.yml' -exec basename {} .yml \; \
        | sort \
        | fzf --prompt='ceedling suite> '
    )"
    test -n "${name}"
    just ceedling::suite "${name}"

# clang-format Ceedling C tests
[group('ceedling')]
format:
    make format-ceedling
