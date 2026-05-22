# Late-Chapter API Drift Cleanup

## Intent

Phase 3 of the conditional migration ran every solution file in the
corpus through `seqc test`. The new failures fell into two distinct
categories: (a) the conditional migration regressions Phase 3 was
chartered to fix — those are now resolved — and (b) **a separate set
of pre-existing failures concentrated in chapters 30–34** caused by
Seq language and stdlib changes that landed without the seqlings
corpus being updated.

The maintainer already noted "anything higher than chapter 14 hasn't
gotten enough scrutiny." This doc is the plan to do that scrutiny in
one focused sweep, decoupled from the 6.0 work, before learners reach
those chapters and hit unexplained red on previously-passing exercises.

## Constraints

- **Don't conflate with the 6.0 conditional migration.** That work is
  in flight and has its own design doc; this is housekeeping for a
  different (older) drift. Fixes in this sweep should not depend on
  6.0-specific syntax.
- **Don't change exercise pedagogy.** If a chapter's exercise teaches
  X, it should still teach X — only the underlying API call sites
  change. Renames in exercise prose can match the new vocabulary, but
  the lesson stays.
- **Out of scope:** introducing new exercises, reorganizing chapters,
  or migrating exercises whose underlying *concept* no longer exists
  in Seq. If a feature was outright removed (no replacement), flag it
  for separate decision rather than silently dropping the exercise.
- **Don't introduce dependencies on unreleased Seq features.** Use
  what the locally-installed RC compiler accepts; if a fix would
  require something newer, file an issue and skip the file.

## Approach

Five-step sweep, gated on `seqc test` per file.

1. **Inventory.** Run `seqc test` against every solution and every
   exercise mode-`test` file. Collect the failure messages, group by
   error class. The Phase 3 sweep already revealed three classes;
   confirm none are missed:
   - `test.assert-eq: stack type mismatch ... cannot unify Int with String`
     (chapters 30, 31, 32, 34) — `test.assert-eq` is now Int-only;
     compare strings with `test.assert-eq-str`.
   - `Undefined word 'map-of'` (chapter 34/01-get) — the map literal
     constructor was renamed or removed; check current stdlib for the
     replacement.
   - `Undefined word 'json.parse'` (chapter 34/04-json) — JSON API
     renamed; check current `std:json` for the new word.
   - Any others surfaced by the inventory.
2. **Cross-reference patch-seq.** For each error class, find the
   replacement in `../patch-seq/crates/compiler/stdlib/` or
   `../patch-seq/docs/`. Document the mapping in this doc as it
   solidifies, so the same pattern doesn't get re-derived per file.
3. **Per-chapter fix sweep.** Apply the mappings file by file. After
   each chapter, re-run `seqc test` for every file in that chapter
   to confirm green — same per-chapter gate the 6.0 migration uses.
4. **Eyeball prose drift.** Where a word is renamed, check the
   chapter's exercise prose, hint files, and README for stale
   references — same eyeball pass we did for the conditional work.
5. **End-to-end verify.** Final `seqlings verify` against a fresh
   `seqlings init` directory with all solutions in place. Every
   exercise reaches `Done`.

## Domain Events

- **Input:** the Phase 3 sweep flagged ~10–15 files across chapters
  30–34 with non-conditional failures.
- **Per file:** `DriftFix { path, error_class }` — one of
  `AssertEqStr`, `MapRename`, `JsonRename`, or `Other`.
- **Aggregate:** `ChapterDriftFixed { chapter, files_changed, all_pass }`.
- **Downstream consequence:** `seqlings update` (already shipped) is
  again the migration path for existing learners. Files they've
  completed against the broken older API will turn red on next watch
  if they got that far — same "good gotcha" as the 6.0 work, surfacing
  exactly the exercises that need attention.
- **Note:** this work is independent of the 6.0 conditional migration.
  Both should be in flight against the same RC compiler so we don't
  fix-then-rewrite-then-fix a single file.

## Checkpoints

1. **Inventory complete.** A short table in this doc lists every
   failing file and its error class. No surprises mid-sweep.
2. **Mapping table populated.** Each error class has one documented
   "old → new" mapping (e.g. `test.assert-eq with strings` →
   `test.assert-eq-str`), so per-file fixes are mechanical.
3. **Each touched chapter green individually.** Per-chapter
   `seqc test` passes for every solution before moving on.
4. **Full verify is green.** Fresh `seqlings init`, copy in all
   solutions, run `seqlings verify` — every exercise reaches `Done`.
5. **No silent removals.** If a feature was removed without a
   replacement, that exercise is flagged in `ROADMAP.md` rather than
   deleted; pedagogical decisions about it are made deliberately.
6. **No keyword regressions.** A grep for `\bthen\b` in code
   (comment-stripped) stays at zero — the 6.0 migration's gains
   aren't undone by this sweep.
