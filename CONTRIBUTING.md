# Contributing

## Note on LLM technologies

This codebase has used LLM technologies to support writing it.

Its core design and architecture were iterated on by hand through v0.6.0
 and this design has been stable since then.

The distinctive pieces are architectural:

- the key `System` abstraction (`src/key.rs`),
- and the `libsmart_keymap` C interface,
- Nickel keymap codegen (`ncl/keymap-codegen.ncl`) (as inspired by fak/kirei).

Much of the per-feature code (tap-hold, layered keys, and so on)
 follows directly from that model.

LLM agents have been used for work that extends an established pattern,
 such as:

- PR code reviews,
- adding or extending tests
   (Rust integration tests, Nickel codegen fixtures, Cucumber scenarios),
- Rust procedural macros and build-script glue,
- adding new key definitions counterparts to the core Rust lib.

LLM coding agents improved in capability
 after the this project's v0.12.0 release,
 and so more code has been written using LLM coding agents since then.

The codebase's functionality is ensured through extensive automated testing.

## Development tasks (`just`)

Day-to-day commands are exposed through the root [justfile](justfile)
([just](https://just.systems/)). Make remains the engine for dependency graphs
and CI; `just` is the human-facing catalog.

Agent-oriented notes (including the pre-push checklist and recipe naming) live in
[AGENTS.md](AGENTS.md).

```text
just                              # list recipes (default)
just choose                       # interactive picker (fzf / JUST_CHOOSER)
just check-quick                  # quick pre-push hygiene: fmt, clippy, doc, nickel format
just test-fast                    # daily loop: ncl checks + rust lib + integration
just test                         # full matrix via Make (comprehensive / CI-ish)

just ncl::checks                  # Nickel evaluated_checks
just ncl::snapshot keymap-…       # one codegen snapshot fixture
just ncl::snapshot-pick           # fzf a fixture, then run it
just ncl::save [fixture]          # refresh expected.rs (one or all)
just ncl::format-check            # format Nickel and fail if files would change

just rust::lib                    # smart-keymap-core + smart-keymap unit tests
just rust::integration [module]   # tests/rust integration suite / filter
just rust::cucumber [filter]      # cucumber features (slow)
just rust::clippy                 # workspace clippy (firmware crates excluded)
just rust::doc                    # cargo doc with --deny warnings (core + firmware)

just ceedling::all                # all C/FFI suites
just ceedling::suite keyboard     # one suite (make test-ceedling-<name>)
just ceedling::suite-pick         # fzf a suite
```

Recipe names use **kebab-case** (`test-fast`, `check-quick`). Module recipes use
Just's **`module::recipe`** form (`ncl::checks`, `rust::clippy`). Paths with `/`
are filesystem only (e.g. `just/ncl.just`), not recipe names.

Before pushing, prefer `just check-quick` so rustfmt, clippy, `cargo doc`
(including firmware packages), and Nickel formatting match CI. Use `just test`
for the full local matrix.

Modules live under `just/` (`ncl`, `rust`, `ceedling`). Firmware packages keep
their own justfiles (e.g. `rp2040-rtic-smart-keyboard`).

CI continues to call `make` / `cargo` directly; keep those entry points stable
when adding recipes.

## License

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
