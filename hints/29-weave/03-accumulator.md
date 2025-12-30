# Hint: Accumulator Generator

This exercise uses the resume value to accumulate state across yields.

## Solution

```seq
: acc-loop ( Ctx Int -- | Yield Int )
    tuck           # ( sum Ctx sum )
    yield          # yield sum, receive increment -> ( sum Ctx increment )
    rot            # ( Ctx increment sum )
    i.add          # ( Ctx new_sum )
    acc-loop       # tail recurse forever
;
```

## Stack Trace

```
Initial: ( Ctx 10 )           # from first strand.resume with 10
tuck:    ( 10 Ctx 10 )        # duplicate sum under Ctx
yield:   ( 10 Ctx 5 )         # yield 10, resume gets 5
rot:     ( Ctx 5 10 )         # rearrange
i.add:   ( Ctx 15 )           # 5 + 10 = 15
acc-loop: recurse with ( Ctx 15 )

Next iteration:
tuck:    ( 15 Ctx 15 )
yield:   ( 15 Ctx 3 )         # yield 15, resume gets 3
rot:     ( Ctx 3 15 )
i.add:   ( Ctx 18 )           # 3 + 15 = 18
...
```

## Why Tail Recursion?

This generator runs forever. Tail call optimization (TCO) ensures the stack doesn't grow - each recursive call reuses the same stack frame.

## Why Cancel?

Infinite generators never complete on their own. `strand.weave-cancel` is required to clean up.
