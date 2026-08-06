# Agent instructions for smart-keymap

Use this file for day-to-day agent work in this repository. Human-oriented
contributing notes live in [CONTRIBUTING.md](CONTRIBUTING.md).

## Task runner

Prefer **`just`** from the repo root. Make is still the engine for CI / dependency
graphs; do not remove or rename Make targets that workflows call.

```text
just                    # list recipes
just choose             # interactive picker
just check-quick        # quick pre-push hygiene (fmt, clippy, doc, nickel format)
just test-fast          # daily tests: ncl checks + rust lib + integration
just test               # full matrix via Make (pre-push / CI-ish)
```

Modules: `just/ncl.just`, `just/rust.just`, `just/ceedling.just`.

### Recipe naming

| Form | Meaning | Examples |
| --- | --- | --- |
| `module::recipe` | Just **submodule** path (language feature) | `ncl::checks`, `rust::clippy` |
| `kebab-case` | Multi-word **recipe** names | `test-fast`, `check-quick`, `fmt-check` |
| `path/to/file` | **Filesystem** only (mod paths, scripts) | `mod ncl "just/ncl.just"` |

Root recipes such as `test-ncl` / `test-rust` are convenience aliases over
module recipes (`ncl::all`, `rust::all`). Prefer either form; do not invent a
third separator style.

## Before you push (common CI failures)

Pushes often fail on **lint/format/doc**, not on the unit tests agents run by
default. After substantive edits — and **before pushing** — run:

```sh
just check-quick
```

That is the quick pre-push surface:

| Check | Recipe | What CI enforces |
| --- | --- | --- |
| rustfmt | `just rust::fmt-check` | `cargo fmt --all -- --check` |
| clippy | `just rust::clippy` | workspace clippy with `-D warnings` (firmware crates excluded on host) |
| cargo doc | `just rust::doc` | `RUSTDOCFLAGS=--deny warnings` for core **and** firmware packages |
| Nickel format | `just ncl::format-check` | `make ncl-format` then clean tree for whitelisted `.ncl` files |

Fix failures before push:

```sh
just fmt                # write rustfmt + nickel format
# then re-run: just check-quick
```

Firmware doc is a frequent surprise: host `cargo test` can pass while
`cargo doc` fails on embedded packages (`rp2040-rtic-smart-keyboard`,
`stm32f4-rtic-smart-keyboard`, `keyberon-smart-keyboard`, etc.).
`just check-quick` / `just rust::doc` cover the same packages CI docs.

When changing Nickel under `ncl/` (or other whitelisted paths), always format
before commit. CI runs format and fails if the tree is dirty.

## Broader verification

| Goal | Command |
| --- | --- |
| Fast daily loop (no full lint matrix) | `just test-fast` |
| Full local matrix (NCL + Rust + Ceedling + lint + cross builds) | `just test` |
| Nickel only | `just test-ncl` / `just ncl::checks` / `just ncl::snapshots` |
| Rust only (CI-ish suites) | `just test-rust` |
| Firmware clippy (target-specific; not in host `just rust::clippy`) | see `.github/workflows/rust-stm32f4.yaml` and `rust-thumbv6m-none-eabi.yaml` |

`just test` is the comprehensive local stand-in for CI. Prefer `just check-quick`
when you mainly need to avoid the format/clippy/doc footguns, then `just test`
when the change is large or touches codegen, FFI, or firmware.

## Layout (short)

| Path | Role |
| --- | --- |
| `smart-keymap-core/` | Core keymap library |
| `smart_keymap/` / root package | C-facing / facade crates |
| `ncl/` | Nickel keymap language, codegen, format whitelist |
| `tests/ncl/`, `tests/rust/` | Snapshot fixtures and Rust integration tests |
| `keyberon-smart-keyboard/`, `rp2040-…`, `stm32…` | Rust firmware packages |
| `firmware/` | C firmware (CH32X, CH58x) |
| `.github/workflows/` | CI source of truth for exact flags |

## Conventions

- Keep changes focused; match existing style in the file you edit.
- Prefer extending existing Make / just recipes over one-off shell in docs.
- CI calls `make` / `cargo` directly; keep those entry points stable.
- For commit and PR conventions, use the user-scope **git-workflow** skill.
