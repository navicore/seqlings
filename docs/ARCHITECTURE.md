# Architecture

## Context & Scope

Seqlings is a CLI teaching tool for the Seq programming language, modeled on Rustlings. A learner edits `.seq` files in their own editor; the CLI watches the filesystem, re-verifies each exercise on change, and reports status.

**Inside the boundary:** the Rust binary, the embedded exercise/solution/hint corpus, and `exercises/info.toml` (the exercise manifest).

**Outside the boundary:**
- The user's editor (any editor — Seqlings does not integrate).
- The Seq compiler `seqc`, invoked as a subprocess (`seqc lint`, `seqc test`). Must be on `PATH`. Its source lives in a separate repo, `navicore/patch-seq`.
- The user's filesystem: the `exercises/`, `solutions/`, `hints/` trees under the CWD.

The corpus doubles as an acceptance test suite for `seqc` — exercise failures sometimes surface bugs in the compiler, not in the exercise.

## Solution Strategy

- **Rust single-binary CLI**, shipped via `cargo install` / `cargo run`. Single crate, three modules.
- **Shell out to `seqc`** rather than link against it — keeps the Seq toolchain loosely coupled.
- **Embed the exercise corpus** with `include_dir!` so `seqlings init` can extract a fresh tree to a new directory without a separate download step.
- **TOML manifest** (`exercises/info.toml`) is the single source of truth for exercise ordering and mode; the filesystem tree is a parallel structure derived from paths in the manifest.
- **Polling watch loop** (250ms `sleep` + mtime scan). `notify` is in `Cargo.toml` but the current code polls.

## Building Blocks

| Module | Responsibility |
|---|---|
| `src/main.rs` | CLI (`clap`), subcommand dispatch, watch loop, display/formatting, `StatusCache` (mtime-keyed) |
| `src/exercise.rs` | `Exercise`, `ExerciseMode { Compile, Test }`, `ExerciseStatus { Done, NotDone, CompileError, TestFail }`; `load_exercises()` parses `exercises/info.toml`; derives `hint_path()` / `solution_path()` by mirroring the exercise path |
| `src/runner.rs` | `compile()` → `seqc lint <path>`; `run_tests()` copies the file to a temp path prefixed `test-` (required by `seqc test`) and runs `seqc test` |

**Corpus layout** (parallel trees — an exercise at `exercises/S/N.seq` has its solution at `solutions/S/N.seq` and its hint at `hints/S/N.md`):

```
exercises/info.toml         # ordered manifest, defines mode per exercise
exercises/<section>/<N-name>.seq
solutions/<section>/<N-name>.seq
hints/<section>/<N-name>.md
```

**Key invariants:**
- An exercise's status is determined by this fixed flow: `# I AM NOT DONE` present → `NotDone`; else `seqc lint` fails → `CompileError`; else (when `mode = "test"`) `seqc test` output contains `FAIL`/`panicked` or exits non-zero → `TestFail`; else → `Done`.
- `StatusCache` is keyed on (path, mtime). Any edit invalidates the entry; the short-circuit `# I AM NOT DONE` check happens before any subprocess call.
- Ordering in `info.toml` is the ordering the learner progresses through — the watch loop always targets the first non-`Done` exercise.

## Crosscutting Concepts

- **Errors to the user** are `String`-typed, formatted with `colored`, and printed inline. Fatal startup errors (`load_exercises` failure, missing `info.toml`) `process::exit(1)` after a hint. Subprocess failures surface `stdout + stderr` verbatim.
- **Embedded content** (exercise/solution/hint trees) lives in the binary via `include_dir!` and is only materialized on disk by `seqlings init`. Released binaries don't need the source tree at hand.
- **Terminal output** uses the `colored` crate; screen clears use a raw ANSI escape (`\x1B[2J\x1B[1;1H`). No TUI framework.
- **Filesystem watching** is polling: every 250 ms the loop scans exercise mtimes and redraws if any changed recently. The `notify` / `notify-debouncer-mini` deps are not currently wired in.
- **Path resolution** is relative to CWD. `Exercise` paths come straight from `info.toml`; hint/solution paths are derived structurally — the corpus must keep the parallel-tree convention for derivation to work.
