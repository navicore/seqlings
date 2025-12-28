# Hint: Accumulator Pattern

Carry the result-so-far as a parameter.

## Solution

```seq
: sum-to-acc ( Int Int -- Int )
    over 0 <= if
        nip   # Return accumulator
    else
        over i.+      # acc + n
        swap 1 i.- swap   # n-1, new-acc
        sum-to-acc
    then
;

: sum-to ( Int -- Int )
    0 sum-to-acc
;
```

## Why This Matters

The recursive call is the **last thing** that happens - nothing needs to be done after it returns. This is tail recursion, and compilers can optimize it to a loop.

## The Transformation

Non-tail: `result = n + sum(n-1)` - must wait for recursive result
Tail: `sum(n-1, acc+n)` - pass partial result forward

This pattern turns recursion into iteration under the hood.
