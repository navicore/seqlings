# Show Inputs of Failed Assertions

## Intent

When a learner's exercise fails, `seqc test` prints only `test-X ... FAILED` — no indication of *which* `test.assert-eq` failed, what the actual value on the stack was, or what was expected. The learner has no signal beyond "something is off," so they retry blindly or fall back to `seqlings hint`. Showing actual vs expected (and the source line) turns a dead end into a teachable moment and reduces hint usage to cases that actually need conceptual help.

## Constraints

- **Seqlings must not link against the Seq runtime.** The `seqc` subprocess boundary is load-bearing (architecture: loosely coupled; `seqc` lives in `patch-seq`). Any runtime-stack introspection belongs in `seqc`, not in Seqlings.
- **Exercise files stay untouched.** Instrumenting `test.assert-eq` by rewriting the student's code would leak into the file view in their editor and contaminate git diffs.
- **Output size stays bounded.** Current display truncates to 20 lines; that ceiling is about fitting a terminal, not about hiding detail. Don't blow past it.
- **Out of scope:** a TUI, step-through debugging, rendering stack contents over time. Just make a single failing assertion legible.

## Approach

Two-part plan. The real fix lives in `patch-seq`; Seqlings adds a small bridge so failing exercises are still useful while that work lands.

**1. `seqc`-side (primary, in `patch-seq`):** enrich `test.assert-eq` and `test.assert` failure output to include expected value, actual value, and source line number. Approximate shape:

```
test-mutual ... FAILED
  at line 23: expected 8, got 13
```

This is the correct layer for the change — it benefits every `seqc test` consumer, not just Seqlings, and it's the only place with a live stack.

**2. Seqlings-side (bridge, optional):** when a test fails, parse the exercise's test word and list the `test.assert-eq` / `test.assert` calls it contains. This shows the student *what must be true* — not actual values, but the shape of the contract. Once `seqc` ships richer output, this remains useful as labelled context alongside it.

Example target display:

```
  Status: Tests Failed

  test-mutual ... FAILED
    at line 23: expected 8, got 13       ← from seqc (primary)

  Assertions in test-mutual:             ← from seqlings source scan (bridge)
    line 19: test.assert-eq     (expected 0)
    line 23: test.assert-eq     (expected 8)   ← this one failed
    line 27: test.assert-eq     (expected 21)
```

If `seqc` output lacks the line number, the Seqlings-side list still gives the student the assertion contract; if `seqc` provides it, the two parts reinforce each other.

## Domain Events

Current flow:

- `ExerciseChanged(path)` → `TestRun(path)` → `TestFail { stdout: String }`
- Display consumes `TestFail.stdout` verbatim.

Proposed flow:

- `TestFail` grows structured payload: `{ test_name, line_number: Option<u32>, expected: Option<String>, actual: Option<String>, raw_output }`.
- A new event on the Seqlings side: `AssertionsExtracted(test_name) → Vec<AssertionSite { line, expected }>` sourced from the exercise file text.
- Display merges both: structured seqc data when present, assertion-site list always.

The key invariant: the "what went wrong" information must still render as plain text under the current 20-line cap. If `seqc` emits too much, Seqlings filters to the first failure's context.

## Checkpoints

1. **`patch-seq` prototype:** running `seqc test` on a known-failing exercise emits `at line N: expected X, got Y` for `test.assert-eq`.
2. **Seqlings picks it up unchanged.** No Seqlings code change needed to display enriched output — the existing `run_tests()` forwards `stdout + stderr`. Eyeball a real exercise failure; verify the new line is visible.
3. **Bridge works in isolation:** with `seqc` still on its old output, Seqlings parses `exercises/09-recursion/05-mutual.seq`, finds the three `test.assert-eq` calls in `test-mutual`, and prints their line numbers and expected values under the "Tests Failed" header.
4. **Legibility:** for the `05-mutual` case in the prompt, the learner sees at minimum *which* assertion failed and *what value it wanted* — enough to know if they computed the wrong number or terminated the wrong branch.
5. **No regression:** exercises that pass are unchanged; `just ci` stays green; the 20-line output cap holds.

## Open Question

Is there appetite in `patch-seq` for changing `test.assert-eq` output format? If yes, the primary lands there and the bridge can be deferred or skipped. If no, the bridge becomes the whole feature and is a clear Seqlings-only change.
