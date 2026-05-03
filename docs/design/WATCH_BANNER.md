# Watch Banner (ASCII Art Header)

## Intent

Make `seqlings watch` feel like ziglings: every redraw starts with a clear screen and a fixed ASCII-art header for the project, followed by the current exercise assessment.

The banner gives the watch loop a stable visual anchor — the learner's eye lands in the same place each time the screen redraws, and the assessment lives in a consistent region below.

## Constraints

- **Don't break the status flow.** Status detection (`# I AM NOT DONE` → lint → test) and `StatusCache` semantics stay exactly as they are. This is presentation-only.
- **Don't pull in a TUI framework.** Stay on raw ANSI escapes + `colored`, consistent with the rest of `main.rs`.
- **Don't widen the binary's data dependencies.** The art is a string literal in `src/main.rs` (or a small `src/banner.rs`), not a file loaded at runtime.
- **Out of scope:** changing other commands' output (`list`, `hint`, `verify`, `next`, `reset`); making the banner configurable; theming; box-drawing/Unicode art (keep it 7-bit ASCII so it renders in any terminal).
- **Don't redraw on every poll tick.** The current loop only redraws when a file changed in the last 500 ms — preserve that. The banner shouldn't cause flicker.

## Approach

Three small changes in `src/main.rs`:

1. **Add a `print_banner()` helper** that writes a multi-line ASCII-art block (the user-supplied "seqlings" figlet) plus the tagline `"Look out! Broken programs below!"`, colored with `colored` (e.g. green bold for the art, dimmed for the tagline). Pure stdout, no flush concerns beyond the existing pattern.
2. **Clear + banner on initial display.** Today `cmd_watch` prints the welcome lines and a progress indicator before the first `display_current_exercise`. Replace the welcome block with: warm the cache silently (or with a single transient line), then `clear_screen()` once, then `print_banner()`, then `display_current_exercise()`. This makes the first frame structurally identical to every later frame.
3. **Banner on every redraw.** In the change-detected branch (`if changed { clear_screen(); display_current_exercise(...) }`), insert `print_banner()` between the clear and the display call.

The banner string is the figlet block from the user's prompt, embedded as a `const BANNER: &str = "..."`. Name in the art: **seqlings** (matches the binary name and the ziglings analogue). Tagline preserved verbatim.

No changes to `cmd_watch`'s control flow, polling cadence, or cache semantics. `display_current_exercise` is untouched.

## Domain Events

This is a UI-only change; no new domain events. The existing event sequence is unchanged:

- `WatchTickDetectedChange` (a polled mtime fell within the 500ms window) → already triggers redraw. Now the redraw additionally renders the banner first.
- `WatchStarted` (entry to `cmd_watch`) → previously printed welcome text; now clears and renders banner before the first assessment.

Nothing downstream consumes these — they're purely terminal output.

## Checkpoints

1. `cargo run -- watch` shows: cleared screen → ASCII banner → exercise assessment, with no leftover "Welcome to..." or warmup-dots noise above the banner.
2. Editing an exercise file triggers a redraw within ~500ms; the banner reappears in the same position; the assessment below it reflects the new status.
3. Completing an exercise (removing `# I AM NOT DONE`, fixing lint/tests) advances to the next exercise on the next redraw — banner stays put, assessment region updates.
4. `cargo run -- list` / `hint` / `verify` / `next` / `reset` print exactly what they printed before this change (no banner leakage into other commands).
5. `just ci` passes (fmt, clippy `-D warnings`, tests, release build). No new warnings from the embedded multi-line string literal.
6. Banner renders correctly in a narrow terminal (≥ 50 cols). It does not need to be responsive, but it should not corrupt the layout when wrapped — verify by resizing.
