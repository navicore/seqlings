# Hint: Min and Max

`min` and `max` each consume two integers and push one (the smaller or the larger respectively). Stack effects: `( a b -- result )`.

For `range`, the formula is `max(a, b) - min(a, b)`. The challenge: each call to `min` or `max` consumes both inputs, so you can't just chain them — you need to compute one, set it aside, recover the original two values, compute the other, then subtract.

The shape:

1. **Preserve the original pair.** `over over` duplicates both values (`( a b -- a b a b )`), so you have a working copy.
2. **Compute max** on the top pair — consumes two, leaves `( a b max )`.
3. **Bring the original pair back to the top** with `rot rot` (a couple of rotations move the buried pair up over the max result).
4. **Compute min** on the now-top pair — leaves `( max min )`.
5. **Subtract** with `i.-` to get the range.

## Why not save with aux?

You could stash the max on the aux stack instead of using `rot rot`, but it's not necessary here — the values fit comfortably on the data stack and `rot rot` keeps everything visible. Aux is most useful when stack juggling would otherwise need 3+ swaps; for two values, plain data-stack moves are fine.
