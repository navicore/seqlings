# Hint: Accumulator Pattern

The transformation in question: turn recursion that *waits for its result and combines* into recursion that *passes the partial result forward*.

The naive shape (what you'd write reflexively):

```
sum-to(n) = n + sum-to(n-1)
```

The `+` happens AFTER the recursive call returns. So every call has unfinished work waiting on the call stack.

The tail-recursive shape (what this exercise teaches):

```
sum-to-acc(n, acc) = sum-to-acc(n - 1, acc + n)   until n hits zero
sum-to-acc(0, acc) = acc
```

Nothing happens after the recursive call. The "+" was done BEFORE — folded into the new accumulator value that gets passed in.

For the helper, the stack going in is `( n acc )`:

- **Base case**: when n is 0 or below, the answer is whatever's in `acc`. Drop n, leave acc.
- **Recursive case**: compute `acc + n` to get the new accumulator, decrement n, recurse with `( n-1, new-acc )`. The recursive call must be the very last thing.

The wrapper `sum-to` kicks off with `acc = 0`, which is already done in the stub.

## Why this matters

The compiler sees that the recursive call is the LAST action — there's no pending arithmetic to do after it returns. That means each call can REUSE the current stack frame instead of pushing a new one. Tail-recursive code runs in constant stack space, which is the whole point. The next exercise drives this home with 100,000 iterations.
