# `seqlings update` — refresh untouched exercises in place

## Intent

I (the maintainer) iterate on exercise prose, hints, and test code in the seqlings source repo and ship new releases. My own working copy of `my-seqlings/` is partially completed; I want to absorb upstream improvements to the *exercises I haven't touched* without losing my in-progress work or having to redo the chapters I'm past. Today the only options are nuke-and-redo (`rm -rf my-seqlings && seqlings init`) or a hand merge per file. A single command that grabs upstream changes for untouched files and leaves the rest alone is the missing piece.

This is fundamentally a maintainer-ergonomics feature, not a normal learner workflow. Learners can use it too, but the gotcha (see Constraints) is more interesting to authors than to learners.

## Constraints

- **Never silently overwrite work the user has touched.** This is the safety property — it must hold under default invocation. An explicit `--force <name>` may opt out per file.
- **No new dependencies.** The embedded `EXERCISES_DIR` already holds the canonical "fresh" content. Compare against that.
- **Out of scope:** anything that looks like version control (no merging, no three-way diff). If a file has been touched, we skip — period.
- **Out of scope:** detecting renames upstream. A renamed exercise looks like a delete + add to this code. Default behavior creates the new file; the old (now-orphan) file stays on disk untouched.
- **Hints and solutions are not user-edited** by convention, so they refresh wholesale on every `update`. Only exercises themselves get the touch-detection logic.

## Approach

A new `seqlings update` subcommand that walks the embedded `EXERCISES_DIR` and, for each file, decides one of:

| On-disk state | Action |
|---|---|
| File missing | **Create** — extract from embedded |
| Content == embedded | **AlreadyCurrent** — no-op |
| Content differs AND `# I AM NOT DONE` still present | **Replace** — user signaled "not started" via the marker; safe to update prose, comments, test bodies |
| Content differs AND marker removed (status Done / CompileError / TestFail) | **Preserve** — user is in flight or has completed; report and leave alone |

The marker is the user's "I haven't started" signal. When it's present, we trust it. When it's gone, we trust that the user has work in flight (even if it doesn't yet pass).

Solutions (`SOLUTIONS_DIR`) and hints (`HINTS_DIR`) refresh unconditionally — they're reference material, not user files.

Flags:
- `--dry-run` — print the action plan, write nothing.
- `--force <name>` — clobber a specific exercise even if touched. Repeatable.

Output is a summary grouped by action class, listing each affected file. The summary is the deliverable; silent success is anti-feature for this workflow.

## Domain Events

- **Input**: `UpdateRequested { force: Vec<ExerciseName>, dry_run: bool }`
- **Per file**: `UpdateAction { path, kind: Created | Replaced | AlreadyCurrent | PreservedInProgress | PreservedCompleted | ForceReplaced }`
- **Aggregate**: `UpdateSummary { created, replaced, current, preserved_in_progress, preserved_completed }`
- **Side effects**:
  - Writes to `exercises/`, `solutions/`, `hints/` under CWD.
  - Invalidates the on-disk file mtime, which means watch mode (next time it polls) re-evaluates status for replaced files.
- **Important downstream consequence (the "good gotcha"):** previously-completed exercises whose upstream test bodies changed will not be touched by `update`, but on the next watch evaluation may now show as `TestFail`. This is the desired surface area for the maintainer — it flags exactly the exercises whose test contracts moved under their feet, without quietly clobbering their prior solution.

## Checkpoints

1. **Fresh project.** After `seqlings init`, run `seqlings update` → reports `N already up to date`, no files modified.
2. **Mixed state.** With three exercises in different states — (a) marker untouched, (b) marker removed but no other edits, (c) completed solution — bump the binary, run `seqlings update`. Expected: (a) replaced, (b) preserved as in-progress, (c) preserved as completed. Verify on disk.
3. **`--dry-run`.** Same scenario, with `--dry-run` → identical report, zero file writes (verify via `mtime`).
4. **`--force`.** `seqlings update --force 09-recursion/05-mutual` replaces the file even though it had user edits; report classifies it as `ForceReplaced`.
5. **Hints/solutions refresh unconditionally.** Hand-edit a hint, run `update`, verify the hint is restored from embedded.
6. **The good gotcha fires.** Complete an exercise; release a binary that adds an assertion to that exercise's test body; run `update`; the file is preserved; on next `seqlings watch`, that exercise turns red. (This is the desired behavior, not a bug.)
