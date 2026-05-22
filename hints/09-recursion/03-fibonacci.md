# Hint: Fibonacci

The new wrinkle: `fib(n) = fib(n-1) + fib(n-2)` — TWO recursive calls. The first time you've needed to juggle the stack between sibling recursive calls.

There's a cute base case: `fib(0) = 0` and `fib(1) = 1`. If you check `n <= 1`, the result IS just n — so the base-case branch can do *nothing at all* (no `drop`, no push). The original n is already on the stack and it's the right answer. That's a nice property of this particular recurrence.

For the recursive branch, the shape is:

1. Compute `fib(n-1)` — leaves a result on the stack.
2. Reach back to the original n (which is now buried under the first result).
3. Compute `fib(n-2)`.
4. Add the two results.

Step 2 is the key — you need `n` again to compute `n-2`, so step 1 must work from a `dup`'d copy. Between the two recursive calls, a `swap` brings the buried n back to the top so you can derive `n-2` from it.

## Efficiency footnote

This naive implementation is exponentially slow — the same `fib(k)` is recomputed many times. `fib(40)` makes roughly a billion calls. The accumulator pattern (next exercise) is the standard fix.
