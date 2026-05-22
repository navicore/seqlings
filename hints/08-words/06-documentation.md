# Hint: Clamp via Helpers

The trick is the one the exercise spells out: build `min` and `max` helpers first, then `clamp` falls out. If you find yourself writing nested `if`s inside `clamp`, you're fighting the chapter's lesson.

## Building min / max

`min` takes two ints and returns the smaller. The shape: keep both values around long enough to compare them, swap them into a known order if needed, then drop the loser.

The `2dup` + comparison + conditional-swap pattern handles this in one line. The library word `when` (from `include std:control`) is a one-armed `if` — it runs its quotation only when the Bool on top is true. Perfect for "swap if out of order."

`max` is the same shape with the opposite comparison.

## Composing clamp

Once you have `min` and `max`, ask yourself: clamping `value` into `[lo, hi]` is the same as `max(lo, min(value, hi))`. Read that out loud — it's English: "the max of `lo` and the min of `value` and `hi`."

The stack going in is `( value lo hi )`. Your job is to feed those into a `min` call and then a `max` call in the right order. A `rot` and a `swap` get them lined up; the helpers do the rest.

That's the whole point of helper words: the "complex" stack problem dissolves once you factor it. The `clamp` body, with good helpers, should be three or four tokens total.
