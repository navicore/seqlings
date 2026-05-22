# Hint: Countdown

This is the canonical recursive shape: base case + recursive case, picked by an `if`. The exercise prose shows the template — your job is to fill in four blanks.

For `countdown(n)`:

- **Base case** trigger: when does the recursion stop? When n hits 0 (or below).
- **Base case** result: what's the sum when there's nothing left to count? 0.
- **Recursive step** input: what smaller problem? `countdown(n-1)`.
- **Recursive step** combine: how do you build the full answer? Add n to the smaller answer.

The one subtle bit: each branch of the `if` runs against whatever was on the stack going IN. The comparison consumes n, so you need `dup` *before* the check — both branches end up needing the original n. The base branch needs to drop it and push 0; the recursive branch needs n alive for `n-1` and again for the final addition.

That's the shape. The exact tokens are the comparison primitives and arithmetic operators you already know.
