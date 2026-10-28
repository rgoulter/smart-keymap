# Agent instructions for smart-keymap

Use this file for day-to-day agent work in this repository.
Human-oriented contributing notes live in [CONTRIBUTING.md](CONTRIBUTING.md).

## Task runner

Prefer **`just`** from the repo root.
Make is still the engine for CI / dependency graphs;
 do not remove or rename Make targets that workflows call.

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

Root recipes such as `test-ncl` / `test-rust` are convenience aliases
 over module recipes (`ncl::all`, `rust::all`).
Prefer either form;
 do not invent a third separator style.

## Before you push (common CI failures)

Pushes often fail on **lint/format/doc**,
 not on the unit tests agents run by default.
After substantive edits - and **before pushing** - run:

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

Firmware doc is a frequent surprise:
 host `cargo test` can pass while
 `cargo doc` fails on embedded packages
 (`rp2040-rtic-smart-keyboard`, `stm32f4-rtic-smart-keyboard`, `keyberon-smart-keyboard`, etc.).
`just check-quick` / `just rust::doc` cover the same packages CI docs.

When changing Nickel under `ncl/` (or other whitelisted paths),
 always format before commit.
CI runs format and fails if the tree is dirty.

## Broader verification

| Goal | Command |
| --- | --- |
| Fast daily loop (no full lint matrix) | `just test-fast` |
| Full local matrix (NCL + Rust + Ceedling + lint + cross builds) | `just test` |
| Nickel only | `just test-ncl` / `just ncl::checks` / `just ncl::snapshots` |
| Rust only (CI-ish suites) | `just test-rust` |
| Firmware clippy (target-specific; not in host `just rust::clippy`) | see `.github/workflows/rust-stm32f4.yaml` and `rust-thumbv6m-none-eabi.yaml` |

`just test` is the comprehensive local stand-in for CI.
Prefer `just check-quick` when you mainly need to avoid the format/clippy/doc footguns,
 then `just test` when the change is large or touches codegen, FFI, or firmware.

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

### Git and PRs

- **Organised, atomic commits** - one logical change each.
  Not one commit of everything, and not a new commit per tweak or review comment.
- **Amend or squash** `"fix"`, `"address review"`, `"typo"`, and `"oops"`
  into the commit they belong to.
  An open PR is still work in progress:
  prefer rewriting the branch (amend, squash, restack, force-push)
  over stacking fixup commits.
- **Prefer several smaller, focused PRs**
  over one large PR when the work naturally splits.
- Clear scope and title; no unrelated drive-by changes.
  Don't expand an approved PR - open a follow-up.
- **Attribute agent-assisted commits with trailers** -
  use `git commit --trailer` / `git interpret-trailers`
   to add trailers attributing both the model and the agent harness
   (e.g. `Model: <model-name>` and `Agent: <harness-name>`).
  In addition to `Model`/`Agent` trailers,
   also add `Co-Authored-By:` for each, since GitHub surfaces `Co-Authored-By` in the UI
  (e.g. `Co-Authored-By: Muse Code powered by Meta Muse Spark <muse-code@meta.com>`
   and `Co-Authored-By: muse-spark-1.2 <muse-spark@meta.com>`).

### Comment style

Comments document the **interface** of the thing they sit on
 (function, type, field, module): what callers may assume,
 how it relates to other types in this repo, and when to use it.

Start each new sentence on its own comment line.
Wrap a long sentence with a two-space hanging indent
 (the style already used in rustdoc here):

```rust
    /// Physical inputs enter a delay line first so at most one is processed
    ///  per tick, including while a key is pending.
    /// While pending, that delay line is the pending session's ingest queue;
    ///  otherwise it is the global input queue.
```

Use names that exist in the tree: types, fields, modules, tests, rustdoc.
Do not import wording from chat transcripts, review threads,
 or other sources that are not checked in
 (no "as discussed", "handoff", "phase N/stage N/decision X", "the PR plan",
  or coinages that are not types or documented concepts here).
Use an in-repo path or a public name (e.g. ZMK `quick-tap-ms`) instead.

### Rust style

This crate's Rust is **expression-oriented**:
 functions compute and return values.
Prefer that over mutating locals and returning at the end.

Do:

- Use `if` / `match` as expressions
  (`if cond { a } else { b }`, not assign-then-return).
- Dispatch on enums with `match`.
- Prefer iterator chains (`map`, `filter`, `fold`, `collect`) over index loops.
- Name intermediate values with `let` when it helps readability.
- Use small constructor helpers that return values
  (`KeyEvents::event`, `NewPressedKey::key`, `LayerBitset::insert`).

Mutation is fine when it is the point:

- State machines (`Context`, pending / pressed key state).
- `const fn` / `no_std` constraints that cannot use iterators
  (e.g. a `while` in a `const fn`).

### Tests

Non-trivial unit and integration tests should be readable as
**name → AAA comments → code**.

Use these comments:

```rust
// Assemble -- tap-hold key on index 0
// Act -- press, then release before timeout
// Assert -- reports a tap (A), not a hold
```

- `// Assemble`, `// Act`, `// Assert` on every non-trivial test.
- Add a short description after `--` (or `-` / `:`) so the comments
  themselves make the test obviously correct.
- Trivial one-liners (a single `assert_eq!` on a pure helper) may skip
  AAA.

Existing tests already follow this (see `tests/rust/`,
`smart-keymap-core/src/keymap.rs`, `smart-keymap-full-system-std/tests/`).

### Nickel style

Nickel here is also expression-oriented: `|>` pipelines, `let … in`,
record merge (`&`), and `match` for shapes.

- **Prefer `match` over `if`/`else`** for branching — record shapes,
  array cons (`[]` / `[x, ..xs]`), and tagged unions (`'Ok`, `'Press`).
- `if` is fine for a boolean predicate
  (`if std.is_array v then 'Ok else 'Error`)
  or a cheap `std.record.has_field` default.
- Keep extra **deep record-pattern** match arms cheap. Adding another
  `{ layer_modifier = { … } }` style arm to a large `match` can make
  Nickel's match compilation hang or time out.
  Handle extra cases inside an existing arm rather than adding another
  deep pattern (see comments in `ncl/layered-key.ncl`).
