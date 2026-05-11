# Hint: Accumulator Generator

This exercise uses the resume value to accumulate state across yields.

## Solution

```seq
: acc-loop ( T Int -- | Yield Int )
    tuck           # ( sum T sum )
    yield          # yield sum, receive increment -> ( sum T increment )
    rot            # ( T increment sum )
    i.add          # ( T new_sum )
    acc-loop       # tail recurse forever
;
```

## Stack Trace

```
Initial: ( T 10 )           # from first strand.resume with 10
tuck:    ( 10 T 10 )        # duplicate sum under T
yield:   ( 10 T 5 )         # yield 10, resume gets 5
rot:     ( T 5 10 )         # rearrange
i.add:   ( T 15 )           # 5 + 10 = 15
acc-loop: recurse with ( T 15 )

Next iteration:
tuck:    ( 15 T 15 )
yield:   ( 15 T 3 )         # yield 15, resume gets 3
rot:     ( T 3 15 )
i.add:   ( T 18 )           # 3 + 15 = 18
...
```

## Why Tail Recursion?

This generator runs forever. Tail call optimization (TCO) ensures the stack doesn't grow - each recursive call reuses the same stack frame.

## Why Cancel?

Infinite generators never complete on their own. `strand.weave-cancel` is required to clean up.
