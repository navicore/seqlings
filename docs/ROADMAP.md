# Roadmap

## Current State

- 37 sections, ~192 exercises covering Seq from stack basics through combinators, concurrency, and Little's Law / Amdahl's Law.
- Single-binary Rust CLI: `watch` (default), `list`, `hint`, `reset`, `verify`, `next`, `init`.
- Corpus embedded in the binary; `init` extracts it to a fresh directory.
- CI pipeline set up (`just ci` — fmt, clippy `-D warnings`, tests, release build) on Linux PRs.
- mdBook docs source (`book.toml`, `docs/`, `scripts/generate-docs.sh`) is in the repo, but the auto-deploy workflow was removed during the GitHub→Forgejo migration. A self-hosted replacement is sketched in `docs/design/SELF-HOSTED-MDBOOK-PAGES.md`.
- Dual purpose: the exercises serve as an informal acceptance suite for `seqc` (the Seq compiler in `navicore/patch-seq`), and have surfaced real compiler bugs.

## Known Gaps / TODOs

Grep for `TODO` in `src/`:

- **`cmd_reset` inserts `# NOT DONE`**, but the rest of the code uses `# I AM NOT DONE` as the marker — reset does not produce a file the status flow will recognize as unfinished. It also only re-marks the current file; it does not restore the original stub. Comment in `main.rs` notes: "Store originals separately, for now just add back # NOT DONE."
- **`cmd_next` is a stub.** The exercise name is printed but nothing marks the current exercise skipped, so `watch` loops right back to it.

## Near-Term Opportunities

- **Switch watch from polling to event-driven.** `notify` and `notify-debouncer-mini` are already in `Cargo.toml`; the 250 ms polling loop in `cmd_watch` can be replaced with debounced notifications.
- **Follow new Seq features.** When `patch-seq` ships a new built-in or combinator (see recent: `dip`/`keep`/`bi`, `list.reverse`, `string.join`, `map.each`/`map.fold`, `i.neg`), add an exercise section that covers it. The combinators section (37) is the template.

## Long-Term: Self-Hosting

`docs/SELF-HOSTED-PLAN.md` tracks the aspiration to rewrite Seqlings in Seq itself. This doubles as a forcing function for Seq language features (process spawning, `fs.watch`, `chan.select`, terminal colors, signal handling, TOML parsing). Not scheduled — waiting on the listed features to land in `patch-seq`.
