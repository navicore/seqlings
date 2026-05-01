# Migrating Seqlings to Seq 6.0 — `if/else/then` → `if`/`when`/`unless`

## Intent

Seq 6.0 removes the `if`/`else`/`then` keywords and replaces them with a
combinator-based form (`cond [ then-branch ] [ else-branch ] if`, plus
the one-armed shortcuts `when` and `unless`). The full transformation
spec lives in `../patch-seq/docs/MIGRATION_6_0.md`.

The local Seq compiler already speaks the new form, so every Seqlings
exercise, solution, and hint that uses the keyword form will fail to
compile until migrated. Chapter 07 (Conditionals) is more than a
search-and-replace target — its whole pedagogy is the keyword syntax,
so it needs both code and prose rewritten around the new mental model.
The bigger risk is that conditional usage is scattered across many
later chapters' exercises and solutions; a piecemeal rewrite has to
cover all of them or the curriculum stops mid-stream when a learner
hits an unmigrated file.

## Constraints

- **Don't break the test contract.** Every exercise that currently has
  a working solution must still pass against the new compiler after
  migration. The migration spec promises identical type-checking
  semantics for the literal-quotation form, so this should hold —
  verify it.
- **Don't change exercise pedagogy outside chapter 07.** Migrate the
  syntax in place; keep the same skill being taught (recursion, list
  ops, channels, etc.). Conditionals are vocabulary the learner already
  brought from chapter 07.
- **Out of scope:** introducing a new chapter on combinators of `if`.
  The combinator-based form gets taught in chapter 07's rewrite. We're
  not reordering the curriculum.
- **Out of scope:** a Rust-side migration tool. The transformation is
  mechanical enough for hand-application file-by-file under `just ci`
  guard, and patch-seq's own type-checker is the safety net.
- **Don't ship until verified end-to-end.** Every migrated exercise
  must compile and pass under the new compiler before we tag a
  Seqlings release for the 6.0 timeframe.

## Approach

Phased work, gated by `just ci`-equivalent compile checks at each step.

1. **Inventory.** Grep `exercises/`, `solutions/`, `hints/`, and
   exercise `README.md` files for `if`/`else`/`then` keyword usage.
   Count call sites per chapter; flag the chapters that need attention
   beyond the obvious chapter 07 rewrite.

2. **Chapter 07 rewrite.** This is the only chapter that *teaches*
   conditionals, so it carries the new mental model. Rewrite the four
   exercises plus the `README.md`:
   - `01-if-then.seq` → teach `cond [ A ] when` (one-armed)
   - `02-if-else.seq` → teach `cond [ A ] [ B ] if` (two-armed)
   - `03-nested.seq` → teach inside-out nesting
   - `04-fizzbuzz.seq` → real-world combinator chain
   The README and Examples blocks introduce `if` as a stack-effect
   word (`( ..a Bool [..a -- ..b] [..a -- ..b] -- ..b )`) — explicitly
   row-polymorphic, since chapter 11 will reinforce that. Mention
   `when` / `unless` as one-armed shortcuts.

3. **Mechanical sweep of the rest of the corpus.** Apply Rule 1 / Rule 2
   from the migration doc to every other `.seq` file under `exercises/`
   and `solutions/`. Most call sites are inside arithmetic, string, or
   recursion exercises — the bodies are short and the rewrite is local.

4. **Hint and README sweep.** Hint files often show worked solutions
   using the old syntax. Same Rule 1 / Rule 2 transformation. Watch
   for prose lines like "use if/then" — those need rewriting too, not
   just the code blocks.

5. **Per-chapter compile gate.** After migrating each chapter, run
   `seqc test` against every exercise's *solution* file in that chapter
   (the canonical "passing" form) to confirm compile + tests stay green.
   Use the per-chapter walk to bail early if anything regresses.

6. **End-to-end gate.** Final pass: `seqlings verify` against a fresh
   `seqlings init` directory with all solutions copied into place,
   confirming every exercise transitions to `Done`.

## Domain Events

- **Upstream input:** `patch-seq@6.0` ships, removing `if`/`else`/`then`
  as parser keywords. Learners on the new compiler will hit
  `CompileError` on every unmigrated exercise.
- **Per file:** `MigrationApplied { path, kind: TwoArmed | OneArmed |
  Nested }`. Aggregate per chapter.
- **Per chapter:** `ChapterMigrated { chapter, files_changed, all_pass }`.
  All chapters must reach `all_pass = true` before the seqlings release.
- **Downstream consequence:** `seqlings update` (which we just shipped)
  becomes the recommended path for existing learners — replaces every
  exercise stub they haven't started, preserves their in-progress work,
  and on the next watch their previously-completed exercises compiled
  against 5.x will turn red, signalling exactly which ones they need to
  port. That's the "good gotcha" working as designed.

## Checkpoints

1. **Inventory complete.** A short table in this doc (or a comment in
   the PR) lists every chapter and how many `if/else/then` call sites
   it has. No surprises mid-migration.
2. **Chapter 07 reads well.** A reader new to Seq, walking through the
   four rewritten exercises in order, can articulate why `if` is a
   combinator and when to reach for `when` vs `unless` vs the
   two-armed form.
3. **Each migrated chapter compiles solo.** Before moving to the next
   chapter, `seqc test` passes against every solution file in the
   current chapter.
4. **Full-corpus verify is green.** A fresh `seqlings init`, copy in
   all solutions, run `seqlings verify` — every exercise reaches
   `Done` on the new compiler.
5. **Migration log.** A short note in `ROADMAP.md` records that
   6.0-keyword migration happened; a learner upgrading sees a one-liner
   rather than mysterious red exercises.
6. **Rollback story.** If patch-seq 6.0 slips, the migrated branch
   doesn't merge until the compiler is on a public release. The current
   release stays compatible with seqlings on disk.

## Phase 1 Inventory

Counts are **comment-stripped** (`#…` removed before matching) and
use word-boundary regex on `then`, which is a reliable proxy since
`then` is removed entirely in 6.0.

### Code keyword sites (definitive)

| Chapter            | exercises | solutions |
|--------------------|----------:|----------:|
| 07-conditionals    | 1         | 8         |
| 08-words           | 0         | 2         |
| 09-recursion       | 0         | 7         |
| 14-variants        | 0         | 1         |
| 15-lists           | 0         | 1         |
| 30-encoding        | 0         | 2         |
| 31-regex           | 0         | 2         |
| 32-compression     | 0         | 2         |
| 34-http-client     | 0         | 4         |
| **Total**          | **1**     | **29**    |

Only **30 code-level call sites** across the whole corpus, in **9
chapters**. Almost all live in solutions (which carry the canonical
working answer); the lone exercise-side site is in chapter 07.

### Prose / Examples (need review, not pure search-and-replace)

These counts are noisier — they include English uses of the words.
Treat as a list of files to **eyeball** during the sweep, not as a
mechanical edit count:

- **Exercise comment-lines** (`# ...` examples, hint blurbs, etc.)
  mentioning `if`, `else`, or `then`: ~30 chapters touched, biggest
  offenders are `07-conditionals` (32 lines), `20-files` (21),
  `26-tcp` (15), `31-regex` (9).
- **Chapter README files**: 19 README files mention the keyword
  form. `07-conditionals/README.md` (20 mentions) is the deep
  tutorial; `20-files/README.md` (8) and `34-http-client/README.md` (7)
  are next.
- **Hint files**: ~50 mention `if`/`else`/`then`; most are English.
  Real keyword examples in hints are concentrated under `07`, `09`,
  `16`, `20`, `34`.

### Chapter 07 special treatment

Chapter 07 is the only chapter that *teaches* the keyword form, so
even though the code-site count is small (1 + 8), the rewrite has
the biggest blast radius:

- 1 keyword in an exercise (the only exercise-side site corpus-wide)
- 8 keywords in solutions
- 32 comment-line mentions in exercises (Example blocks teaching the
  syntax)
- 20 README mentions
- 5 single-comment-line `if … then` syntax demonstrations

Phase 2 starts here.

### What stays clean

Chapters with **zero `then` keyword sites** in either exercises or
solutions: 00, 01, 02, 03, 04, 05, 06, 10, 11, 12, 13, 16, 19, 20,
21, 22, 23, 24, 25, 26, 27, 28, 29, 33, 35, 36, 37. Some have
`if`/`else`/`then` in *English* comments; those still need an
eyeball pass during Phase 4 but no code change.

> **Surprise:** chapter `20-files` and `26-tcp` show no `then`
> keyword in code despite high comment-line counts. Both READMEs
> talk about error-handling patterns using if/then in prose
> examples; their actual exercise/solution code uses other forms
> (match, status Bools dropped without branching, etc.).
