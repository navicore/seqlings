# Solutions & Hints Audit

## Intent

Recent line-walking through chapters 35–38 surfaced multiple correctness
and pedagogy defects in shipped material:

- **Solutions that don't run** (ch 36 ex 02 hung forever; ch 36 ex 05
  had a stack-effect type identifier the parser rejects).
- **Hints whose code blocks won't compile** (ch 38 used `List` in stack
  effects throughout — a parse error in seqc).
- **Hints/stubs that leak the literal answer** (ch 38 ex 04/05/06 had
  the full solution as a comment in the stub body).
- **Exercise prose with non-compiling examples** (ch 37 ex 06's
  `"score" 42 [ int->string ] dip` is a type mismatch).

These were found by chance. The same classes of defect almost
certainly exist in chapters we haven't re-walked. The goal: sweep the
whole corpus systematically and fix what we find, before more learners
hit them.

Two distinct quality concerns:

1. **Correctness** — solutions pass `seqc test`; stubs lint clean;
   code blocks inside hints and exercise prose compile if extracted.
2. **Pedagogy** — hints scaffold understanding rather than spoon-feed
   the answer; stubs make it clear where the user types and what
   they're solving.

## Constraints

- **Don't change what an exercise teaches.** A reverse exercise should
  still teach reverse. We're cleaning correctness and pedagogy, not
  reshuffling the curriculum.
- **Don't weaken tests** (CLAUDE.md rule). If a solution fails, fix
  the solution or the worker idiom, never the assertion.
- **No new exercises in this sweep.** New exercises (like ch 02 ex 07
  `aux`) are separate work, even when they're motivated by audit
  findings.
- **Pedagogy is subjective; have a rubric.** A hint showing the
  complete word body is the answer. A hint sketching strategy +
  pointing at the relevant primitive is not. The rubric (below) keeps
  reviewers consistent.
- **Out of scope:** redesigning a chapter (gets its own design doc, as
  ch 35 did), fixing seqc bugs surfaced along the way (file upstream),
  unrelated drift like task #32 (`time.now` units in ch 23 + ch 36).
- **Don't depend on unreleased Seq features.** What ships in the
  locally-installed compiler is the ceiling.
- **`seq:allow(...)` pragmas are themselves a leak.** A line like
  `# seq:allow(unchecked-chan-receive)` at the top of a channels
  exercise tells the learner the answer uses `chan.receive`. So
  "just suppress the warning" isn't a free fix. Tactics in priority
  order: (a) rewrite the example so the warning doesn't fire (e.g.
  actually check the Bool); (b) accept the warning as part of the
  stub's surface — the lint UX will note it but it doesn't break
  status; (c) only as a last resort, add the pragma and accept the
  spoiler. Document the choice when (c) is used.

## Approach

Two tracks. The mechanical correctness sweep is scripted and
re-runnable; the pedagogy review of hints is a one-time human walk.

### Track A — mechanical (scripted)

`scripts/audit-curriculum.sh` runs:

- `seqc test` against every `solutions/**/*.seq` (renamed to
  `test-*.seq` in a tmpdir per existing convention).
- `seqc lint` against every `exercises/**/*.seq` (stubs), errors only
  — warnings on stubs are sometimes intentional (see the
  `seq:allow(...)` constraint above).

Supports `--chapter NN` for per-chapter gating. Findings:

- `SolutionFails` / `SolutionTimeout` (test failed or hung).
- `StubLintError` (the stub the learner first sees has a hard error).

A scripted hint-compile pass was prototyped and rejected: too many
false positives from hint blocks that are inline usage demos rather
than top-level definitions (e.g. `7 [ 1 i.+ ] keep` as a one-line
illustration). Those don't lint at the top level, but they're not
defects.

### Track B — human (one-time hint audit)

Walk every `hints/**/*.md` once, chapter by chapter, and assess
against the rubric below. Not scripted because the signal is
qualitative ("does this teach or spoon-feed?"). A separate
`AUDIT_HINTS_FINDINGS.md` is the working ledger — one row per hint
file with verdict and notes.

Rubric:

- **Allowed:** naming the primitive(s) to reach for; describing the
  recursion shape ("branch on `empty?`; trivial case returns acc;
  recursive case bumps and recurses"); a stack trace showing state
  evolution; a 1–2 token snippet where it *is* the lesson; the
  complete word body *with* context that explains why each step is
  there.
- **The smell:** a complete `: word ... ;` body that's identical or
  near-identical to the solution, followed by little or no prose
  explaining *why*. If a reader could delete every line of prose and
  still copy-paste a working answer, the hint is solution-leak, not
  teaching.
- **Sub-3-token carve-out:** words whose entire body is one or two
  tokens (e.g. `0 variant.field-at`) can't avoid showing the body —
  the body IS the lesson. Skip those.

Per-hint verdict: `Teaches`, `Leaks` (rewrite required),
`Borderline` (rewrite if convenient, leave if not), `N/A` (sub-3-token
carve-out).

### Sequencing

1. **Track A first**, full corpus. Triage findings into actual
   defects vs. drift the audit is the wrong venue for (e.g. task #32
   `time.now` units). Fix the defects per-chapter, re-running the
   script per chapter as a gate.
2. **Track B after Track A is green** — easier to focus on pedagogy
   when no correctness noise is in the way. Walk the chapters in
   order; record verdicts in `AUDIT_HINTS_FINDINGS.md`; rewrite
   `Leaks` hints in a second pass.
3. **End-to-end verify.** `seqlings verify` on a fresh `seqlings
   init`; script returns zero findings; the hints ledger is fully
   filled in.

## Domain Events

- **Per file:** `AuditFinding { path, class }` where `class` is one of
  the six listed above.
- **Per chapter:** `ChapterAudited { chapter, findings_resolved,
  files_changed }`. Stays open until the script returns zero findings
  for that chapter.
- **Aggregate:** `CurriculumAudited { all_solutions_green,
  all_stubs_lint_clean, all_hint_code_compiles, pedagogy_pass }`.
- **Downstream consequence:** learners who had been red on a buggy
  solution-style issue will see it clear after `seqlings update`. The
  pedagogy fixes are mostly hidden behind preserved files (no marker
  removal triggered), so most users won't see them turn red — they'll
  just notice the next time they hit the hint.
- **Spillover into other work:** correctness findings may surface
  upstream `seqc` bugs (precedent: patch-seq#499). File those
  separately; the audit doesn't wait on them — it works around with
  fixed-shape solutions and notes the issue.

## Checkpoints

1. **Audit script landed** at `scripts/audit-curriculum.sh`.
   Returns nonzero on findings; supports `--chapter NN`.
2. **Track A inventory committed** as a sibling
   `AUDIT_FINDINGS_MECHANICAL.md`. Every finding categorized; drift
   that belongs to other tracks (e.g. task #32) explicitly excluded
   with a one-line reason.
3. **Per-chapter Track A sweeps completed**, each gated on the script
   returning zero findings for that chapter.
4. **Track B ledger started** as `AUDIT_HINTS_FINDINGS.md`. One row
   per hint file with verdict.
5. **Track B rewrites completed** for every hint marked `Leaks`.
6. **Final `seqlings verify` green** on a fresh init with all
   solutions dropped in.
7. **Audit script in CI** (or documented in `docs/ROADMAP.md` as a
   pre-release step) so future correctness drift is caught before it
   ships. Pedagogy regressions can't be caught this way and are
   accepted as a one-time cleanup.
