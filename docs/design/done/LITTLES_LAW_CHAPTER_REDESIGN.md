# Chapter 35 (Little's Law) Redesign

## Intent

Chapter 35 claims to teach Little's Law (`L = λW`), but four of its five
exercises only verify a sum and never measure the quantity they describe.
The prose talks about latency, throughput, queue depth, and capacity, while
the tests check `104 test.assert-eq`. We want the exercises to actually
exhibit and measure the quantities — so a learner ends the chapter with
empirical intuition for `L`, `λ`, and `W`, not just a stack-shuffling drill
that happens to be themed around them.

Secondary motivator: ex 2 currently hangs (only 2 one-shot workers for 4
items), and the workaround patterns in ex 2 / ex 4 use `2dup ... drop drop`
boilerplate that's mechanically redundant — `strand.spawn` already preserves
the parent stack. That style is teaching a wrong mental model. Cleaning it
up alongside the conceptual rework avoids two passes through the chapter.

## Constraints

- **No new Seq features.** Use what `seqc` already exposes: `chan.make`,
  `chan.send`, `chan.receive`, `strand.spawn`, `time.now`, `time.sleep-ms`.
- **No reliance on `chan.close` to terminate receive loops.** Upstream bug
  `navicore/patch-seq#499`: `chan.close` does not unblock a subsequent
  `chan.receive` — receive blocks forever instead of returning
  `( default false )` as the ch 24 README and ex 04 prose document. Confirmed
  in both single-strand and cross-strand reproducers under `seqc 7.4.0`.
  Recursive workers loop-on-success-flag depend on this, so we stay with
  fixed-count one-shot workers until #499 lands.
- **Don't break the surrounding curriculum.** Chapter 36 (Amdahl's Law)
  reuses the worker-strand idiom; whatever shape we settle on for ch 35
  should be coherent with the ch 36 exercises so a learner doesn't see two
  conflicting patterns back to back.
- **Test runtime must stay reasonable** under `seqc test`. Each exercise
  should complete in well under a second; sleep budgets shouldn't add up to
  noticeable test-suite drag.
- **Timing asserts must be tolerant.** Wall-clock checks in CI are flaky if
  they pin to tight windows. Assert on ratios or generous bounds (e.g.
  `elapsed < 4 × per-item-cost` for parallel; `elapsed > 0.9 × N ×
  per-item-cost` for serial), not exact ms.
- **Out of scope:** changing `seqc`, changing the test harness, retiring
  the chapter, or expanding to >5 exercises. Hint files and prose get
  rewritten to match; the exercise count stays at 5.

## Approach

One structural change carries the redesign:

**Every exercise produces and asserts on a measurable Little's-Law quantity,
not on a hand-summed result.** Worker shape stays as one-shot strands
(`[ over chan.receive drop ...process... swap chan.send drop drop ]` —
already established in ch 25 ex 5 and ch 35 ex 5). Worker count equals
item count K=N in each exercise. The teaching value comes from `time.now`
bracketing and asserts on derived quantities, not from worker-loop
mechanics.

- ex 1 — **W (latency)**: time a single item, assert `elapsed >= per-item-cost`.
  Already in this shape; minor polish only.
- ex 2 — **λ (throughput)**: spawn N one-shot workers, send N items,
  bracket with `time.now`, assert `N / elapsed` is within a generous
  parallel window (e.g. wall time < `2 × per-item-cost` for full
  parallelism). Drop the `104 test.assert-eq` sum check.
- ex 3 — **L (queue depth)**: 1 worker, N items, learner maintains an
  "in-flight" counter (incremented on send, decremented on receive),
  sample periodically, average the samples, assert `L > some-floor` under
  sustained overload. (No `chan.depth` primitive; the counter is the
  whole point.)
- ex 4 — **verify L = λW**: run a controlled experiment, compute all
  three independently, assert `abs(L - λ*W) < tolerance`. The exercise
  that pays off the previous three.
- ex 5 — **capacity planning**: given a target λ and measured W, derive
  `K = ceil(λW)`, spawn K workers, verify achieved λ meets target.

Prose, hints, and headers get rewritten in lockstep with the test
assertions so the lesson matches what runs. The K=N coupling is
acknowledged in the chapter README as a teaching simplification, with a
forward-reference to a future "free-running concurrency" chapter that can
land once `chan.close` works as documented.

## Domain Events

- **Input:** chapter 35 currently 5 exercises, all passing the test harness
  except ex 2 (hang). All five "pass" tests that don't measure their stated
  topic.
- **Per exercise:** `ExerciseRedesigned { num, quantity_measured, test_shape }`
  — each redesigned exercise commits with a test that asserts on the named
  quantity (W, λ, L, L=λW, or required-K), not on a sum.
- **Aggregate:** `ChapterRedesigned { all_solutions_green, all_exercises_red_until_filled }`.
- **Downstream consequence:** `seqlings update` pushes the new chapter to
  existing learners. Any learner who'd previously marked ch 35 exercises
  Done will see them turn `NotDone` (the `# I AM NOT DONE` marker returns
  with the new stub). This is intentional and acceptable — the prior pass
  didn't teach what the new pass teaches.
- **Curriculum spillover:** chapter 36 (Amdahl's Law) reuses ch 35's
  worker idiom. If ch 35 adopts a recursive worker, ch 36's spawn lines
  should be re-shaped to match in the same change or immediately after.

## Checkpoints

1. **Spike outcome recorded.** ✅ Spike confirmed `chan.close` doesn't
   unblock `chan.receive` in `seqc 7.4.0`; filed as `navicore/patch-seq#499`.
   Approach pivoted to fixed-count one-shot workers.
2. **Per-exercise solution measures its quantity.** Each of `solutions/35-
   littles-law/0N-*.seq` computes the named Little's-Law quantity and
   asserts on it. No `104 test.assert-eq` left in the chapter.
3. **Tests are tolerant.** Run each solution 20× back to back; no flakes.
   If a timing assert is sharper than that survives, it's too sharp.
4. **Exercise stubs lint clean.** With `# I AM NOT DONE` removed, every
   stub still passes `seqc lint` (the `list-of` regression we hit recently
   doesn't recur).
5. **Chapter 36 stays coherent.** A quick eyeball pass: chapter 36's
   spawn/receive idioms either match ch 35's new shape or are explicitly
   marked as a different teaching pattern with a one-line comment.
6. **Full `seqlings verify` green** against a fresh `seqlings init` with
   all solutions dropped in.
7. **Follow-up tracked.** When `patch-seq#499` is fixed, revisit this
   chapter to swap the K=N one-shot pattern for free-running recursive
   workers — that's the better teaching shape and the chapter README's
   forward-reference promises it.
