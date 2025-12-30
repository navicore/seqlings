# Hint: Basic Weave

A weave body receives `(Ctx, resume_value)` on the stack and must thread the Ctx through yield.

## Solution

```seq
: yield-42 ( Ctx Int -- Ctx | Yield Int )
    drop           # drop resume value, stack: ( Ctx )
    42 yield       # yield 42, receive next resume -> ( Ctx Int )
    drop           # drop the resume response -> ( Ctx )
    # implicit return - weave completes
;
```

## How It Works

1. `strand.weave` creates the weave and waits for first `strand.resume`
2. `0 strand.resume` sends 0 to the weave, which wakes up with `( Ctx 0 )`
3. We `drop` the 0, push 42, and `yield` it
4. Caller receives `( Handle 42 true )` - the 42 and `true` for has_more
5. Caller does `0 strand.resume` again
6. Weave wakes from yield with the new value, drops it, and returns
7. Caller receives `( Handle 0 false )` - placeholder and `false` for done
